
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}")]
async fn index(info: web::Path<(String, )>) -> impl Responder {
    format!("Input: {}!", info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}