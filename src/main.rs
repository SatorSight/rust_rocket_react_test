#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_include_static_resources;

extern crate rocket_contrib;
//extern crate tera;
extern crate dotenv;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod models;
mod schema;
mod database;
mod structs;
//use tera::Context;
//use tera::Tera;
//use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use serde_json::*;
use rocket_contrib::json::Json;

use rocket_include_static_resources::StaticResponse;
use database::all_users_stagings;

#[post("/add_user", format = "application/json", data = "<params>")]
fn add_user(params: Json<structs::UserParams>) -> Result<String> {
    let _username = &params.name;
    let user = database::create_user(_username.to_string());
    let res = serde_json::to_string(&user);
    return res
}

#[delete("/user", format = "application/json", data = "<params>")]
fn delete_user(params: Json<structs::UserDeleteParams>) -> Result<String> {
    let _user_id = &params.user_id;
    database::destroy_user(*_user_id);
    serde_json::to_string("ok")
}

#[post("/add_staging", format = "application/json", data = "<params>")]
fn add_staging(params: Json<structs::StagingParams>) -> Result<String> {
    let staging_name = &params.name;
    let staging = database::create_staging(staging_name.to_string());
    let res = serde_json::to_string(&staging);
    return res
}

#[delete("/staging", format = "application/json", data = "<params>")]
fn delete_staging(params: Json<structs::StagingDeleteParams>) -> Result<String> {
    let _staging_id = &params.staging_id;
    database::destroy_staging(*_staging_id);
    serde_json::to_string("ok")
}

#[patch("/staging", format = "application/json", data = "<params>")]
fn toggle_staging(params: Json<structs::StagingToggleParams>) -> Result<String> {
    let _staging_id = params.staging_id;
    let _stag = database::find_staging(_staging_id);

    database::toggle_staging_busy(_stag);
    serde_json::to_string("ok")
}

#[post("/assign_staging_to_user", format = "application/json", data = "<params>")]
fn add_staging_to_user(params: Json<structs::UserStagingParam>) -> Result<String> {
    database::create_user_staging(params.user_id, params.staging_id);
    let res = serde_json::to_string("ok");
    return res
}

#[get("/")]
fn index() -> StaticResponse {
    static_response!("index.html")
}

#[get("/all")]
fn all() -> Result<String> {
    let template_data = structs::TemplateData {
        users: database::all_users(),
        stagings: database::all_stagings(),
        users_stagings: all_users_stagings()
    };

    let res = serde_json::to_string(&template_data);
    return res;
}

fn main() {
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "index.html", "dist/index.html",
            );
        }))
        .mount("/", routes![index])
        .mount("/public", StaticFiles::from("./dist"))
        .mount("/api", routes![
            all,
            add_user,
            add_staging,
            add_staging_to_user,
            delete_user,
            delete_staging,
            toggle_staging
        ])
//        .attach(Template::fairing())
        .launch();
}
