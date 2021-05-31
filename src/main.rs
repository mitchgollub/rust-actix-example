mod config;
mod handlers;

use crate::handlers::app_config;
use actix_web::{middleware::Logger, App, HttpServer};
use eyre::Result;
use tracing::info;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = config::Config::from_env().expect("Server configuration");
    info!("Starting server at http://{}:{}/", config.host, config.port);
    HttpServer::new(move || App::new().wrap(Logger::default()).configure(app_config))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    Ok(())
}
