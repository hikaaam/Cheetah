use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::db::{model::User, schema::users};
pub mod model;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
