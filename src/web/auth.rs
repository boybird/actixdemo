use actix_web::{web as a_web, HttpRequest,HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    name: String,
    password: String,
}

pub fn login(req: a_web::Json<LoginForm>) -> impl Responder {
    println!("{:?}, {}, {}",req, req.name, req.password);

    HttpResponse::Ok().json(req.deref())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterForm {
    name: String,
    password: String,
    email: String,
}

pub fn register(req: a_web::Json<RegisterForm>) -> impl Responder {
    
    
    HttpResponse::Ok().json(req.deref())
}
