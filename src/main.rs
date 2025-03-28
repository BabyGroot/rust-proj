use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

// Define a struct for your data
#[derive(Serialize)]
struct Message {
    status: String,
    message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to my Actix API! Try /hello endpoint.")
}

// Create a handler for the hello endpoint
#[get("/hello")]
async fn hello() -> impl Responder {
    let message = Message {
        status: "success".to_string(),
        message: "Hello, API!".to_string(),
    };

    HttpResponse::Ok().json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:4000");

    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("localhost", 4000))? // Configure IP and port here
        .run()
        .await
}
