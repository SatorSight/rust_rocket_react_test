use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::*;

use schema::users;
use schema::stagings;
use schema::users::dsl::*;
use schema::stagings::dsl::*;
use schema::users_stagings::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn find_staging(_staging_id: i32) -> Staging {
    let conn = establish_connection();
    stagings.find(_staging_id).first::<Staging>(&conn)
        .expect("Error loading staging")
}

pub fn all_users() -> Vec<User>{
    let conn = establish_connection();
    users.load::<User>(&conn)
        .expect("Error loading users")
}

pub fn all_stagings() -> Vec<Staging>{
    let conn = establish_connection();
    stagings.load::<Staging>(&conn)
        .expect("Error loading stagings")
}

pub fn all_users_stagings() -> Vec<UsersStaging>{
    let conn = establish_connection();
    users_stagings.load::<UsersStaging>(&conn)
        .expect("Error loading users stagings")
}

pub fn toggle_staging_busy(_staging: Staging) {
    let conn = establish_connection();
    diesel::update(stagings.find(_staging.id))
        .set(busy.eq(!_staging.busy))
        .get_result::<Staging>(&conn)
        .expect("Error toggling staging busy");
}

pub fn create_user_staging(params_user_id: i32, params_staging_id: i32) {
    let conn = establish_connection();
    let new_user_staging = NewUserStaging {
        user_id: params_user_id,
        staging_id: params_staging_id
    };

    diesel::insert_into(users_stagings)
        .values(&new_user_staging)
        .get_result::<UsersStaging>(&conn)
        .expect("Error saving new user_staging");
}

pub fn create_user(username: String) -> User {
    let conn = establish_connection();
    let new_user = NewUser {
        name: username
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(&conn)
        .expect("Error saving new user")
}

pub fn destroy_user(_user_id: i32) {
    let conn = establish_connection();
    diesel::delete(users.filter(users::dsl::id.eq(_user_id)))
        .execute(&conn)
        .expect("Error destroying user");
}

pub fn destroy_staging(_staging_id: i32) {
    let conn = establish_connection();
    diesel::delete(stagings.filter(stagings::dsl::id.eq(_staging_id)))
        .execute(&conn)
        .expect("Error destroying staging");
}

pub fn create_staging(staging_name: String) -> Staging {
    let conn = establish_connection();
    let new_staging = NewStaging {
        name: staging_name
    };

    diesel::insert_into(stagings)
        .values(&new_staging)
        .get_result(&conn)
        .expect("Error saving new staging")
}
