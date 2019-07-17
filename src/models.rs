
use diesel::{self, prelude::*};
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::schema::{users};

use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

// pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);


// impl Actor for DbExecutor {
//    type Context = SyncContext<Self>;
// }

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime, // only NaiveDateTime works here due to diesel limitations
    pub updated_at: NaiveDateTime, // only NaiveDateTime works here due to diesel limitations
}

