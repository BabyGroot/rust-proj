#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

// Define a struct for your data
#[derive(Serialize)]
struct Message {
    status: String,
    message: String,
}

// Create a route that returns JSON
#[get("/hello")]
fn hello() -> Json<Message> {
    Json(Message {
        status: "success".to_string(),
        message: "Hello, Rocket API!".to_string(),
    })
}

// Create a route for the root path
#[get("/")]
fn index() -> &'static str {
    "Welcome to my Rocket API! Try /hello endpoint."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
