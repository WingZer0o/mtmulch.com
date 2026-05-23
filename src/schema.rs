// @generated automatically by Diesel CLI.

diesel::table! {
    otp_codes (id) {
        id -> Uuid,
        user_id -> Uuid,
        code -> Varchar,
        expires_at -> Timestamptz,
        used -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        otp_enabled -> Bool,
    }
}

diesel::joinable!(otp_codes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(otp_codes, users,);
