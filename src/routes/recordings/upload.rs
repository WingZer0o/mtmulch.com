use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use azure_storage_blobs::prelude::{BlobBlockType, BlockId, BlockList, ContainerClient};
use diesel::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::models::recording::NewRecording;
use crate::schema::recordings;

const BLOCK_SIZE: usize = 4 * 1024 * 1024; // 4 MiB per block

#[post("/upload")]
pub async fn upload(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    mut payload: web::Payload,
    container: web::Data<ContainerClient>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let recording_id = Uuid::new_v4();

    let filename = req
        .headers()
        .get("X-Filename")
        .and_then(|v| v.to_str().ok())
        .map(sanitize_filename)
        .unwrap_or_else(default_filename);

    // Blob key is scoped per user so keys never collide across accounts.
    let blob_key = format!("{user_id}/{recording_id}/{filename}");
    let blob_client = container.blob_client(&blob_key);

    let mut uncommitted: Vec<BlobBlockType> = Vec::new();
    let mut pending: Vec<u8> = Vec::new();
    let mut block_idx: u32 = 0;
    let mut total_bytes: i64 = 0;

    // Stream the incoming payload; flush full blocks to Azure as they fill.
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(actix_web::error::ErrorInternalServerError)?;
        total_bytes += chunk.len() as i64;
        pending.extend_from_slice(&chunk);

        while pending.len() >= BLOCK_SIZE {
            let block_data: Vec<u8> = pending.drain(..BLOCK_SIZE).collect();
            let block_id = BlockId::new(block_idx.to_be_bytes().to_vec());
            blob_client
                .put_block(block_id.clone(), block_data)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            uncommitted.push(BlobBlockType::Uncommitted(block_id));
            block_idx += 1;
        }
    }

    // Flush any remaining bytes as the final block.
    if !pending.is_empty() {
        let block_id = BlockId::new(block_idx.to_be_bytes().to_vec());
        blob_client
            .put_block(block_id.clone(), pending)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        uncommitted.push(BlobBlockType::Uncommitted(block_id));
    }

    if uncommitted.is_empty() {
        // Zero-byte upload — write an empty blob directly.
        blob_client
            .put_block_blob(vec![])
            .content_type("audio/flac")
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    } else {
        // Commit all uploaded blocks.
        blob_client
            .put_block_list(BlockList { blocks: uncommitted })
            .content_type("audio/flac")
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    }

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
