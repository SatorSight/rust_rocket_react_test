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
}

#[get("/")]
fn index() -> Result<String> {
    use self::schema::users::dsl::*;
    use self::schema::stagings::dsl::*;

    let connection = establish_connection();
    let user_results = users //.filter(published.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    let stagings_results = stagings
        .limit(5)
        .load::<Staging>(&connection)
        .expect("Error loading stagings");

//    let mut context = Context::new();
//    let mut tera = Tera::default();
//    tera.autoescape_on(vec![]);

    let template_data = TemplateData { users: user_results, stagings: stagings_results };
//    let payload = serde_json::to_string(&template_data).unwrap();
//    println!("{}", payload);
//
//    context.insert("data", &payload);

//    Template::render("index", &context)
//    "lol"
    let res = serde_json::to_string(&template_data);
    return res;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
//        .attach(Template::fairing())
        .mount("/public", StaticFiles::from("./dist"))
        .launch();
}
