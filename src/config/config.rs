use std::env;
use std::error::Error;

pub struct Config {
    pub db_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let server_host = env::var("SERVER_HOST")?;
        let server_port = env::var("SERVER_PORT")?.parse::<u16>()?;

        Ok(Self {
            db_url: database_url,
            server_host: server_host,
            server_port: server_port,
        })
    }
}
