use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
mod api;
use api::campuses::{add_campus, delete_campus_by_id, get_all_campuses, get_campus_by_id};
use env_logger;

#[derive(Serialize, Deserialize)]
pub struct Campus {
    id: i32,
    created_at: String,
    updated_at: String,
    name: String,
    image_url: String,
    address: String,
    description: String,
}

#[derive(Deserialize)]
pub struct GetCampusById {
    id: i32,
}

pub const MAX_SIZE: usize = 262_144;

pub struct AppState {
    campuses: Arc<Mutex<HashMap<i32, Campus>>>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                campuses: Arc::new(Mutex::new(HashMap::new())),
            }))
            .service(
                web::scope("/api")
                    .route("/campuses", web::get().to(get_all_campuses))
                    .route("/campuses", web::post().to(add_campus))
                    .route("/campuses/{id}", web::get().to(get_campus_by_id))
                    .route("/campuses/{id}", web::delete().to(delete_campus_by_id)),
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
