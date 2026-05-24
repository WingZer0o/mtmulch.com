use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::failed_login_attempts)]
pub struct NewFailedLoginAttempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub attempted_at: DateTime<Utc>,
}
