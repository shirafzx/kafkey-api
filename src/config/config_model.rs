#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub min_idle: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct AuthSecret {
    pub secret: String,
    pub refresh_secret: String,
}
