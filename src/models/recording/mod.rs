use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::recordings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recording {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub size_bytes: i64,
    pub data: Vec<u8>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct RecordingMeta {
    pub id: Uuid,
    pub filename: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::recordings)]
pub struct NewRecording {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub size_bytes: i64,
    pub data: Vec<u8>,
}
