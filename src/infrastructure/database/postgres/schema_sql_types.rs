// Custom SQL types for the database schema

#[derive(diesel::sql_types::SqlType)]
#[diesel(postgres_type(name = "timestamptz"))]
pub struct Timestamptz;
