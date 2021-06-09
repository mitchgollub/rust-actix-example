use super::repositories::MoviesRepository;
use actix_web::HttpResponse;
use actix_web::{error, get, web, Result};

#[get("health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/score/{movie_name}")]
pub async fn score(
    movies_repo: web::Data<MoviesRepository>,
    web::Path(movie_name): web::Path<String>,
) -> Result<HttpResponse> {
    let resp = movies_repo.get(movie_name);
    match resp {
        Ok(movie) => Ok(HttpResponse::Ok().json(movie)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}
