//use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Staging {
    pub id: i32,
    pub name: String,
}
