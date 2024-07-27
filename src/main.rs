// use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyResponse {
    message: String,
}

async fn index() -> impl Responder {
    web::Json(MyResponse {
        message: "Hello, world!".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api", web::get().to(index))
            .service(fs::Files::new("/", "./frontend").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
