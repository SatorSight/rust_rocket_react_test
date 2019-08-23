#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

extern crate rocket_contrib;
//extern crate tera;
extern crate dotenv;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod models;
mod schema;

//use tera::Context;
//use tera::Tera;
//use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::*;
use serde_json::*;
use rocket_contrib::json::Json;

use self::schema::users::dsl::*;
use self::schema::stagings::dsl::*;
use self::schema::users_stagings::dsl::*;
//use diesel::pg::expression::dsl::any;
use self::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Serialize, Deserialize)]
struct TemplateData {
    users: Vec<User>,
    stagings: Vec<Staging>,
    users_stagings: Vec<UsersStaging>,
}

#[derive(Deserialize)]
struct UserParams {
    name: String,
}

#[derive(Deserialize)]
struct StagingParams {
    name: String,
}

#[derive(Deserialize)]
struct UserDeleteParams {
    user_id: i32,
}

#[derive(Deserialize)]
struct StagingDeleteParams {
    staging_id: i32,
}

#[derive(Deserialize)]
struct StagingToggleParams {
    staging_id: i32,
}

#[post("/add_user", format = "application/json", data = "<params>")]
fn add_user(params: Json<UserParams>) -> Result<String> {
    let connection = establish_connection();
    let username = &params.name;
    let user = create_user(&connection, username.to_string());
    let res = serde_json::to_string(&user);
    return res
}

#[delete("/user", format = "application/json", data = "<params>")]
fn delete_user(params: Json<UserDeleteParams>) -> Result<String> {
    let connection = establish_connection();

    let _user_id = &params.user_id;
    destroy_user(&connection, *_user_id);
    serde_json::to_string("ok")
}

#[post("/add_staging", format = "application/json", data = "<params>")]
fn add_staging(params: Json<StagingParams>) -> Result<String> {
    let connection = establish_connection();
    let staging_name = &params.name;
    let staging = create_staging(&connection, staging_name.to_string());
    let res = serde_json::to_string(&staging);
    return res
}

#[delete("/staging", format = "application/json", data = "<params>")]
fn delete_staging(params: Json<StagingDeleteParams>) -> Result<String> {
    let connection = establish_connection();

    let _staging_id = &params.staging_id;
    destroy_staging(&connection, *_staging_id);
    serde_json::to_string("ok")
}

#[patch("/staging", format = "application/json", data = "<params>")]
fn toggle_staging(params: Json<StagingToggleParams>) -> Result<String> {
    let connection = establish_connection();
    let _staging_id = &params.staging_id;

    let _stag = stagings.find(1).first::<Staging>(&connection)
        .expect("Error loading staging");

    diesel::update(stagings.find(_staging_id))
        .set(busy.eq(!_stag.busy))
        .get_result::<Staging>(&connection)
        .expect("Error loading staging");

    serde_json::to_string("ok")
}

#[derive(Deserialize)]
struct UserStagingParam {
    user_id: i32,
    staging_id: i32
}

#[post("/assign_staging_to_user", format = "application/json", data = "<params>")]
fn add_staging_to_user(params: Json<UserStagingParam>) -> Result<String> {
    let connection = establish_connection();
    create_user_staging(&connection, params.user_id, params.staging_id);
    let res = serde_json::to_string("ok");
    return res
}

fn create_user_staging(conn: &PgConnection, params_user_id: i32, params_staging_id: i32) {
    let new_user_staging = NewUserStaging {
        user_id: params_user_id,
        staging_id: params_staging_id
    };

    diesel::insert_into(users_stagings)
        .values(&new_user_staging)
        .get_result::<UsersStaging>(conn)
        .expect("Error saving new post");
}

fn create_user(conn: &PgConnection, username: String) -> User {
    let new_user = NewUser {
        name: username
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

fn destroy_user(conn: &PgConnection, _user_id: i32) {
    diesel::delete(users.filter(self::schema::users::dsl::id.eq(_user_id)))
        .execute(conn)
        .expect("Error saving new user");
}

fn destroy_staging(conn: &PgConnection, _staging_id: i32) {
    diesel::delete(stagings.filter(self::schema::stagings::dsl::id.eq(_staging_id)))
        .execute(conn)
        .expect("Error saving new staging");
}

fn create_staging(conn: &PgConnection, staging_name: String) -> Staging {
    let new_staging = NewStaging {
        name: staging_name
    };

    diesel::insert_into(stagings)
        .values(&new_staging)
        .get_result(conn)
        .expect("Error saving new post")
}


#[get("/")]
fn index() -> Result<String> {
//    // TODO: remove
//    use std::thread;
//    thread::sleep_ms(4000);

    let connection = establish_connection();
    let user_results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    let stagings_results = stagings
        .load::<Staging>(&connection)
        .expect("Error loading stagings");

    let us = users_stagings
        .load::<UsersStaging>(&connection)
        .expect("Error loading stagings");


//    let image_tag_ids = ImageTag::belonging_to(img).select(image_tags::tag_id);
//    tags::table
//        .filter(tags::id.eq(any(image_tag_ids)))
//        .load::<Tag>(conn)
//        .expect("could not load tags")

//    let mut context = Context::new();
//    let mut tera = Tera::default();
//    tera.autoescape_on(vec![]);

    let template_data = TemplateData {
        users: user_results,
        stagings: stagings_results,
        users_stagings: us
    };
//    let payload = serde_json::to_string(&template_data).unwrap();
//    println!("{}", payload);
//
//    context.insert("data", &payload);

//    Template::render("index", &context)
//    "_user_id"
    let res = serde_json::to_string(&template_data);
    return res;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            add_user,
            add_staging,
            add_staging_to_user,
            delete_user,
            delete_staging,
            toggle_staging
        ])
//        .attach(Template::fairing())
        .mount("/public", StaticFiles::from("./dist"))
        .launch();
}
