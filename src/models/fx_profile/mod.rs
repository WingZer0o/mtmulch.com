use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::fx_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FxProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub settings: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fx_profiles)]
pub struct NewFxProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub settings: String,
}
