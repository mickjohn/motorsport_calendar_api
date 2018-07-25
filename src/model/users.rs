use super::super::schema::*;

#[derive(PartialEq, Debug, Clone, Queryable, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub hashed_password: String,
}

#[derive(Insertable, FromForm)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub hashed_password: String,
}
