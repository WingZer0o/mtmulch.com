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
        views -> Int8,
    }
}

diesel::table! {
    email_list (id) {
        id -> Uuid,
        email -> Varchar,
        verified -> Bool,
        verification_public_key -> Nullable<Varchar>,
        unsubscribe_public_key -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    failed_login_attempts (id) {
        id -> Uuid,
        user_id -> Uuid,
        attempted_at -> Timestamptz,
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
    password_history (id) {
        id -> Uuid,
        user_id -> Uuid,
        password_hash -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    post_comments (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
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
        is_locked -> Bool,
        unlock_public_key -> Nullable<Varchar>,
    }
}

diesel::joinable!(blog_posts -> users (author_id));
diesel::joinable!(failed_login_attempts -> users (user_id));
diesel::joinable!(otp_codes -> users (user_id));
diesel::joinable!(password_history -> users (user_id));
diesel::joinable!(post_comments -> blog_posts (post_id));
diesel::joinable!(post_comments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog_posts,
    email_list,
    failed_login_attempts,
    otp_codes,
    password_history,
    post_comments,
    users,
);
