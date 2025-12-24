// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::infrastructure::database::postgres::schema_sql_types::*;

    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 50]
        first_name -> Nullable<Varchar>,
        #[max_length = 50]
        last_name -> Nullable<Varchar>,
        active -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::infrastructure::database::postgres::schema_sql_types::*;

    roles (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::infrastructure::database::postgres::schema_sql_types::*;

    permissions (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        resource -> Varchar,
        #[max_length = 50]
        action -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::infrastructure::database::postgres::schema_sql_types::*;

    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::infrastructure::database::postgres::schema_sql_types::*;

    role_permissions (id) {
        id -> Int4,
        role_id -> Int4,
        permission_id -> Int4,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    roles,
    permissions,
    user_roles,
    role_permissions,
);
