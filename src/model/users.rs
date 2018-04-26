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

// pub struct UserWithoutPassword {
//     pub id: i32,
//     pub user_name: String,
// }

// // type UserWithPassword = NewUser;

// impl From<User> for UserWithoutPassword {
//     fn from(user: User) -> Self {
//         UserWithoutPassword {
//             id: user.id,
//             user_name: user.user_name,
//         }
//     }
// }

// // Get the user by name and verify that the password is correct
// pub fn validate_user(
//     user_to_validate: &auth::UserWithPlaintextPassword,
//     conn: &SqliteConnection,
// ) -> Result<UserWithoutPassword, ()> {
//     let u = users::table
//         .filter(users::user_name.eq(&user_to_validate.user_name))
//         .first::<User>(conn)
//         .unwrap();

//     match verify(&user_to_validate.plaintext_password, &u.hashed_password).unwrap() {
//         true => Ok(UserWithoutPassword::from(u)),
//         false => Err(()),
//     }
// }
