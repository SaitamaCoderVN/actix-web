use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    println!("Server starting on port {}", port);
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}