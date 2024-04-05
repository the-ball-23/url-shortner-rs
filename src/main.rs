mod urlstore;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub static URLSTORE: Lazy<Mutex<urlstore::UrlStore>> =
    Lazy::new(|| Mutex::new(urlstore::UrlStore::new()));

async fn shorten(url: web::Json<urlstore::ShortenRequest>) -> impl Responder {
    let short_url = URLSTORE.lock().unwrap().shorten(&url.url);
    HttpResponse::Ok().body(short_url)
}

#[get("/{short_id}")]
async fn redirect(path: web::Path<String>) -> impl Responder {
    let short_id = path.into_inner();
    match URLSTORE.lock().unwrap().redirect(&short_id) {
        Ok(url) => HttpResponse::PermanentRedirect()
            .append_header(("Location", url.clone()))
            .finish(),
        Err(e) => HttpResponse::NotFound().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(
            web::scope("/api")
                .service(web::resource("/shorten").route(web::post().to(shorten)))
                .service(redirect),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
