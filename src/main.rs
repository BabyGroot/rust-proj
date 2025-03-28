#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

// Define a struct for your data
#[derive(Serialize)]
struct Message {
    status: String,
    message: String,
    code: u32,
}

// Create a route that returns JSON
#[get("/hello")]
fn hello() -> Json<Message> {
    Json(Message {
        status: "ok".to_string(),
        message: "Hello, Rocket API!".to_string(),
        code: 200,
    })
}

// Create a route for the root path
#[get("/")]
fn index() -> &'static str {
    "Welcome to my Rocket API! Try /hello endpoint."
}

#[catch(404)]
fn not_found() -> Json<Message> {
    Json(Message {
        status: "not_found".to_string(),
        message: "The requested resource was not found".to_string(),
        code: 404,
    })
}

#[catch(500)]
fn server_error() -> Json<Message> {
    Json(Message {
        status: "internal_server_error".to_string(),
        message: "Something went wrong".to_string(),
        code: 500,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .register("/", catchers![not_found, server_error])
}
