// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        key_hash -> Varchar,
        #[max_length = 20]
        key_prefix -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 20]
        environment -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        last_used_at -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

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
        tenant_id -> Nullable<Uuid>,
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
        tenant_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    tenant_admins (id) {
        id -> Uuid,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        company_name -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        email_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tenant_settings (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        allow_signups -> Nullable<Bool>,
        require_email_verification -> Nullable<Bool>,
        enable_2fa -> Nullable<Bool>,
        session_duration_minutes -> Nullable<Int4>,
        allowed_oauth_providers -> Nullable<Array<Nullable<Text>>>,
        custom_email_templates -> Nullable<Jsonb>,
        #[max_length = 255]
        webhook_url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tenants (id) {
        id -> Uuid,
        owner_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        slug -> Varchar,
        #[max_length = 255]
        domain -> Nullable<Varchar>,
        #[max_length = 255]
        logo_url -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        #[max_length = 20]
        plan_tier -> Nullable<Varchar>,
        max_users -> Nullable<Int4>,
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
        tenant_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    webhook_deliveries (id) {
        id -> Uuid,
        webhook_id -> Uuid,
        #[max_length = 50]
        event_type -> Varchar,
        payload -> Jsonb,
        response_status -> Nullable<Int4>,
        response_body -> Nullable<Text>,
        delivered_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 500]
        url -> Varchar,
        events -> Array<Nullable<Text>>,
        #[max_length = 255]
        secret -> Varchar,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(api_keys -> tenants (tenant_id));
diesel::joinable!(permissions -> tenants (tenant_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(roles -> tenants (tenant_id));
diesel::joinable!(tenant_settings -> tenants (tenant_id));
diesel::joinable!(tenants -> tenant_admins (owner_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_social_accounts -> users (user_id));
diesel::joinable!(users -> tenants (tenant_id));
diesel::joinable!(webhook_deliveries -> webhooks (webhook_id));
diesel::joinable!(webhooks -> tenants (tenant_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    blacklisted_tokens,
    permissions,
    role_permissions,
    roles,
    tenant_admins,
    tenant_settings,
    tenants,
    user_roles,
    user_social_accounts,
    users,
    webhook_deliveries,
    webhooks,
);
