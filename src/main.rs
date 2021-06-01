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
            .service(web::resource("/").route(web::get().to(health)))
            .service(web::scope("/movies").service(score))
    })
    .bind(server_url)?
    .run()
    .await?;
    Ok(())
}
