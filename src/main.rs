#![allow(unused_imports)]

 #[macro_use]
extern crate diesel;
 
 use actix_web::{web as a_web, App, HttpRequest, HttpServer, Responder};

 mod schema;
 mod web;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", a_web::get().to(greet))
            .route("/hi/{name}", a_web::get().to(greet))
            .route("/auth/login", a_web::post().to(web::auth::login))
            .route("/auth/register", a_web::post().to(web::auth::register))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}

