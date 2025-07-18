use diesel::query_dsl::methods::{FindDsl, SelectDsl};
use diesel::{RunQueryDsl, SelectableHelper};

use crate::db::establish_connection;
use crate::db::model::User;
use crate::db::schema::users::dsl::*;

pub fn get_user_by_auth_id(user_id: i64) -> Option<User> {
    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .find(user_id)
        .load(connection);

    match results {
        Ok(mut user) if !user.is_empty() => Some(user.remove(0)),
        Ok(_) => None,
        Err(e) => {
            eprintln!("Error loading user: {}", e);
            None
        }
    }
}
