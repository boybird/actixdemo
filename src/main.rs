#![allow(unused_imports)]

#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
 
 use actix_web::{web as a_web, App, HttpRequest, HttpServer, Responder};
 use actix::prelude::{Addr,SyncArbiter};
 use dotenv::dotenv;
 use std::env;
use diesel::{r2d2::ConnectionManager, PgConnection};

 mod schema;
 mod web;
 mod models;
 mod errors;

 // use models::DbExecutor;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


fn main() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // let address :Addr<DbExecutor>  = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    

    
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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

