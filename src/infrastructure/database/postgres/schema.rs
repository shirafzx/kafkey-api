// @generated automatically by Diesel CLI.

diesel::table! {
    blacklisted_tokens (jti) {
        jti -> Uuid,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        resource -> Varchar,
        #[max_length = 50]
        action -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Uuid,
        permission_id -> Uuid,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
        assigned_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_social_accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 50]
        provider -> Varchar,
        #[max_length = 255]
        provider_user_id -> Varchar,
        #[max_length = 255]
        provider_email -> Nullable<Varchar>,
        access_token -> Nullable<Text>,
        refresh_token -> Nullable<Text>,
        expires_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 30]
        username -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        #[max_length = 255]
        avatar_image_url -> Nullable<Varchar>,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        email -> Varchar,
        is_active -> Nullable<Bool>,
        is_verified -> Nullable<Bool>,
        last_login_at -> Nullable<Timestamptz>,
        failed_login_attempts -> Int4,
        locked_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        verification_token -> Nullable<Varchar>,
        verification_token_expires_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        password_reset_token -> Nullable<Varchar>,
        password_reset_expires_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        two_factor_secret -> Nullable<Varchar>,
        two_factor_enabled -> Bool,
        two_factor_backup_codes -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_social_accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    blacklisted_tokens,
    permissions,
    role_permissions,
    roles,
    user_roles,
    user_social_accounts,
    users,
);
