use actix_web::{web, get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::pokemon::get_pokemon_count;


pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
                web::resource("/")
                    .route(web::get().to(|| { hello() }))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
        );
        cfg.service(
                web::resource("/pokemon_count")
                    .route(web::get().to(|| { pokemon_count() }))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
        );
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}
#[derive(Serialize, Deserialize)]
struct PokemonCount {
    count: u32,
}

pub async fn hello() -> impl Responder {
        HttpResponse::Ok().json(Message {
                message: "Hello, World!".to_owned(),
        })
}
pub async fn pokemon_count() -> impl Responder {
        HttpResponse::Ok().json(PokemonCount {
                count: get_pokemon_count().await,
        })
}