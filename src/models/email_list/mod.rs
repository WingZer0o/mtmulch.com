use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::email_list)]
pub struct EmailListEntry {
    pub id: Uuid,
    pub email: String,
    pub verified: bool,
    pub verification_public_key: Option<String>,
    pub unsubscribe_public_key: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::email_list)]
pub struct NewEmailListEntry {
    pub id: Uuid,
    pub email: String,
    pub verification_public_key: String,
    pub created_at: DateTime<Utc>,
}
