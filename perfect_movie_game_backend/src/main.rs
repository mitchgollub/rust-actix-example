mod config;
mod handlers;
mod repositories;

use actix_web::{middleware::Logger, web, App, HttpServer};
use eyre::Result;
use handlers::*;
use repositories::MoviesRepository;
use tracing::info;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = config::Config::from_env().expect("Server configuration");
    let server_url = format!("{}:{}", config.host, config.port);

    info!("Starting server at http://{}:{}/", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(MoviesRepository::new(&config))
            .service(health)
            .service(web::scope("/movies").service(score))
            .service(actix_files::Files::new("/", &config.static_files).index_file("index.html"))
    })
    .bind(server_url)?
    .run()
    .await?;
    Ok(())
}
