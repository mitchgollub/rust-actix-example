use actix_web::HttpResponse;
use actix_web::{web, web::ServiceConfig};

pub fn app_config(config: &mut ServiceConfig) {
    let health_check = web::resource("/").route(web::get().to(health));

    config.service(health_check);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
