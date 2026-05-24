use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::password_history)]
pub struct PasswordHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::password_history)]
pub struct NewPasswordHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
