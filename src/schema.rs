// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (id) {
        id -> Uuid,
        title -> Varchar,
        slug -> Varchar,
        content -> Text,
        published -> Bool,
        author_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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
        is_verified -> Bool,
        verification_public_key -> Nullable<Varchar>,
        is_admin -> Bool,
    }
}

diesel::joinable!(blog_posts -> users (author_id));
diesel::joinable!(otp_codes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(blog_posts, otp_codes, users,);
