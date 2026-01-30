// @generated automatically by Diesel CLI.

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
    }
}

diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    role_permissions,
    roles,
    user_roles,
    users,
);
