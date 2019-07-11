use super::schema::users;
use super::schema::stagings;
use super::schema::users_stagings;

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

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
#[belongs_to(Staging)]
pub struct UsersStaging {
    pub id: i32,
    pub user_id: i32,
    pub staging_id: i32,
}

#[derive(Insertable)]
#[table_name="users_stagings"]
pub struct NewUserStaging {
    pub user_id: i32,
    pub staging_id: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser{
    pub name: String
}

#[derive(Insertable)]
#[table_name="stagings"]
pub struct NewStaging{
    pub name: String
}
