use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub otp_enabled: bool,
    pub is_verified: bool,
    pub verification_public_key: Option<String>,
    pub is_admin: bool,
    pub is_locked: bool,
    pub unlock_public_key: Option<String>,
    pub profile_image: Option<Vec<u8>>,
    pub profile_image_content_type: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub verification_public_key: String,
}
