use crate::errors::ServiceError;
use actix_web::{web as a_web, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Local};
use diesel::{self, prelude::*};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    name: String,
    password: String,
}

pub fn login(req: a_web::Json<LoginForm>) -> impl Responder {
    println!("{:?}, {}, {}", req, req.name, req.password);

    HttpResponse::Ok().json(req.deref())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterForm {
    username: String,
    name: String,
    password: String,
    email: String,
}

pub fn register(
    req: a_web::Json<RegisterForm>,
    db: a_web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = db.get().unwrap();
    let new_user = crate::models::User {
        username: req.username.clone(),
        email: req.email.clone(),
        password: req.password.clone(),
        created_at: Local::now().naive_local(), // only NaiveDateTime works here due to diesel limitations
        updated_at: Local::now().naive_local(), // only NaiveDateTime works here due to diesel limitations
    };
    diesel::insert_into(users)
        .values(new_user)
        .execute(&conn)
        .map_err(|error| {
            println!("{:#?}", error); // for debugging purposes
            ServiceError::InternalServerError
        })
        .unwrap();
    
    
    HttpResponse::Ok().json(req.deref())
}
