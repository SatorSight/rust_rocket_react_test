use models::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct TemplateData {
    pub users: Vec<User>,
    pub stagings: Vec<Staging>,
    pub users_stagings: Vec<UsersStaging>,
}

#[derive(Deserialize)]
pub struct UserParams {
    pub name: String,
}

#[derive(Deserialize)]
pub struct StagingParams {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserDeleteParams {
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct StagingDeleteParams {
    pub staging_id: i32,
}

#[derive(Deserialize)]
pub struct StagingToggleParams {
    pub staging_id: i32,
}

#[derive(Deserialize)]
pub struct UserStagingParam {
    pub user_id: i32,
    pub staging_id: i32
}
