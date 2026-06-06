use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use azure_storage_blobs::prelude::{BlobBlockType, BlockId, BlockList, ContainerClient};
use diesel::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::db::DbPool;
use crate::helpers::encryption::{BLOCK_SIZE, EncryptionKey, NONCE_SIZE};
use crate::middleware::AuthUser;
use crate::models::recording::NewRecording;
use crate::schema::recordings;

#[post("/upload")]
pub async fn upload(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    mut payload: web::Payload,
    container: web::Data<ContainerClient>,
    enc_key: web::Data<EncryptionKey>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let recording_id = Uuid::new_v4();

    let filename = req
        .headers()
        .get("X-Filename")
        .and_then(|v| v.to_str().ok())
        .map(sanitize_filename)
        .unwrap_or_else(default_filename);

    let blob_key = format!("{user_id}/{recording_id}/{filename}");
    let blob_client = container.blob_client(&blob_key);

    let (nonce, mut encryptor) = enc_key.begin_encrypt();
    let mut uncommitted: Vec<BlobBlockType> = Vec::new();
    let mut pending: Vec<u8> = Vec::new();
    let mut block_idx: u32 = 0;
    let mut total_bytes: i64 = 0;
    // Nonce is prepended once to the first block uploaded.
    let mut nonce_prepended = false;

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(actix_web::error::ErrorInternalServerError)?;
        total_bytes += chunk.len() as i64;
        pending.extend_from_slice(&chunk);

        // Encrypt and upload every full segment immediately.
        while pending.len() >= BLOCK_SIZE {
            let plaintext: Vec<u8> = pending.drain(..BLOCK_SIZE).collect();
            let ct = encryptor
                .encrypt_next(plaintext.as_slice())
                .map_err(|_| actix_web::error::ErrorInternalServerError("Encryption failed"))?;

            let block_data = with_nonce_prefix(&nonce, ct, &mut nonce_prepended);
            put_block(&blob_client, &mut uncommitted, &mut block_idx, block_data)
                .await?;
        }
    }

    // Seal the final segment — may be the only one if the file was < BLOCK_SIZE.
    let ct = encryptor
        .encrypt_last(pending.as_slice())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Encryption failed"))?;

    let block_data = with_nonce_prefix(&nonce, ct, &mut nonce_prepended);
    put_block(&blob_client, &mut uncommitted, &mut block_idx, block_data)
        .await?;

    blob_client
        .put_block_list(BlockList { blocks: uncommitted })
        .content_type("application/octet-stream")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let new_rec = NewRecording {
        id: recording_id,
        user_id,
        filename: filename.clone(),
        size_bytes: total_bytes,
        blob_key,
    };

    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::insert_into(recordings::table)
            .values(&new_rec)
            .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "filename": filename,
        "size_bytes": total_bytes,
    })))
}

/// Prepends the nonce to `ct` on the very first call; returns `ct` unchanged thereafter.
fn with_nonce_prefix(nonce: &[u8; NONCE_SIZE], ct: Vec<u8>, prepended: &mut bool) -> Vec<u8> {
    if *prepended {
        return ct;
    }
    *prepended = true;
    let mut out = Vec::with_capacity(NONCE_SIZE + ct.len());
    out.extend_from_slice(nonce);
    out.extend_from_slice(&ct);
    out
}

async fn put_block(
    blob_client: &azure_storage_blobs::prelude::BlobClient,
    uncommitted: &mut Vec<BlobBlockType>,
    block_idx: &mut u32,
    data: Vec<u8>,
) -> actix_web::Result<()> {
    let block_id = BlockId::new(block_idx.to_be_bytes().to_vec());
    blob_client
        .put_block(block_id.clone(), data)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    uncommitted.push(BlobBlockType::Uncommitted(block_id));
    *block_idx += 1;
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or("").trim();
    let cleaned: String = base
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        .collect();
    let cleaned = cleaned.trim_matches('.').to_string();
    if cleaned.is_empty() || !cleaned.to_ascii_lowercase().ends_with(".flac") {
        default_filename()
    } else {
        cleaned
    }
}

fn default_filename() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("recording-{secs}.flac")
}
