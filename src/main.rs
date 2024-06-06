mod items;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

use items::Items;

async fn get(i: web::Data<Mutex<Items>>) -> impl Responder {
    if let Some(i) = i.lock().unwrap().get() {
        HttpResponse::Ok().body(i.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[derive(Deserialize)]
struct AddBody {
    item: String,
}

async fn add(i: web::Data<Mutex<Items>>, data: web::Json<AddBody>) -> impl Responder {
    i.lock().unwrap().add(data.into_inner().item);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let items = web::Data::new(Mutex::new(Items::new()));
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::Conflict().finish(),
                )
                .into()
            });
        App::new()
            .app_data(items.clone())
            .app_data(json_config)
            .route("/", web::get().to(get))
            .route("/", web::post().to(add))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
