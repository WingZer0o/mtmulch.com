use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use actix_web::web::Bytes;
use async_stream::stream;
use azure_storage_blobs::prelude::ContainerClient;
use diesel::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::db::DbPool;
use crate::helpers::encryption::{BlobDecryptor, ENC_BLOCK_SIZE, EncryptionKey, NONCE_SIZE};
use crate::middleware::AuthUser;
use crate::schema::recordings;

#[get("/{id}")]
pub async fn get_recording(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    container: web::Data<ContainerClient>,
    enc_key: web::Data<EncryptionKey>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let recording_id = path.into_inner();

    let (filename, blob_key) = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        recordings::table
            .filter(recordings::id.eq(recording_id))
            .filter(recordings::user_id.eq(user_id))
            .select((recordings::filename, recordings::blob_key))
            .first::<(String, String)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorNotFound("Recording not found"))?;

    let mut page_stream = container.blob_client(&blob_key).get().into_stream();
    let enc_key = enc_key.clone();

    let byte_stream = stream! {
        let mut nonce_buf: Vec<u8> = Vec::with_capacity(NONCE_SIZE);
        let mut decryptor: Option<BlobDecryptor> = None;
        let mut pending: Vec<u8> = Vec::new();

        while let Some(page_result) = page_stream.next().await {
            let page = match page_result {
                Ok(p) => p,
                Err(e) => {
                    yield Err(actix_web::error::ErrorInternalServerError(e.to_string()));
                    return;
                }
            };
            let mut data = page.data;
            while let Some(chunk_result) = data.next().await {
                let chunk = match chunk_result {
                    Ok(c) => c,
                    Err(e) => {
                        yield Err(actix_web::error::ErrorInternalServerError(e.to_string()));
                        return;
                    }
                };

                // Accumulate the nonce from the first NONCE_SIZE bytes.
                let ct_start = if nonce_buf.len() < NONCE_SIZE {
                    let take = (NONCE_SIZE - nonce_buf.len()).min(chunk.len());
                    nonce_buf.extend_from_slice(&chunk[..take]);
                    if nonce_buf.len() == NONCE_SIZE {
                        decryptor = Some(enc_key.make_decryptor(&nonce_buf));
                    }
                    take
                } else {
                    0
                };
                pending.extend_from_slice(&chunk[ct_start..]);

                // Decrypt every segment we know is non-final: those where at least
                // ENC_BLOCK_SIZE bytes still remain after draining.
                while pending.len() > ENC_BLOCK_SIZE {
                    let block: Vec<u8> = pending.drain(..ENC_BLOCK_SIZE).collect();
                    let result = decryptor.as_mut().unwrap().decrypt_next(block.as_slice());
                    match result {
                        Ok(plain) => yield Ok(Bytes::from(plain)),
                        Err(_) => {
                            yield Err(actix_web::error::ErrorInternalServerError("Decryption failed"));
                            return;
                        }
                    }
                }
            }
        }

        // Authenticate and decrypt the final segment.
        match decryptor.take() {
            Some(dec) => match dec.decrypt_last(pending.as_slice()) {
                Ok(plain) => yield Ok(Bytes::from(plain)),
                Err(_) => yield Err(actix_web::error::ErrorInternalServerError("Decryption failed")),
            },
            None => yield Err(actix_web::error::ErrorInternalServerError("Blob too short for nonce")),
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("audio/flac")
        .insert_header((
            "Content-Disposition",
            format!("inline; filename=\"{filename}\""),
        ))
        .streaming(byte_stream))
}
