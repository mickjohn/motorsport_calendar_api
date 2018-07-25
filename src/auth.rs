use super::model::users::User;
use super::schema::*;
use bcrypt::verify;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket::http::hyper::header::Basic;
use rocket::http::Status;
use rocket::request;
use rocket::request::{FromRequest, Request};
use rocket::response::Responder;
use rocket::Outcome;
use rocket::Response;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Forbidden<R>(pub R);

/// Sets the status code of the response to Forbidden
impl<'r, R: Responder<'r>> Responder<'r> for Forbidden<R> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Response::build_from(self.0.respond_to(req)?)
            .status(Status::Forbidden)
            .ok()
    }
}

#[derive(Debug, PartialEq, Fail)]
pub enum AuthenticationError {
    #[fail(display = "You need to authenticate with Basic auth to access this resource.")]
    NeedBasic,

    #[fail(display = "Incorrect Username or Password")]
    IncorrectUsernameOrPassword,

    #[fail(display = "No Password Provided")]
    NoPassword,

    #[fail(display = "Incorrect Base64")]
    IncorrectBase64,

    #[fail(display = "Database Error")]
    DieselError(DieselError),
}

#[derive(Debug, PartialEq)]
pub struct UserWithPlaintextPassword {
    pub user_name: String,
    pub plaintext_password: String,
}

#[derive(Debug, PartialEq)]
pub struct UserWithoutPassword {
    pub id: i32,
    pub user_name: String,
}

// Extract a user name and password from the basic auth header
impl<'a, 'r> FromRequest<'a, 'r> for UserWithPlaintextPassword {
    type Error = AuthenticationError;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<UserWithPlaintextPassword, AuthenticationError> {
        // Get the Authorization header
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Forbidden, (AuthenticationError::NeedBasic)));
        }

        // Get the user from the Authorization header
        let auth_key = keys[0];
        if auth_key.starts_with("Basic ") {
            match decode_basic_auth(auth_key) {
                Ok(user) => Outcome::Success(user),
                Err(err) => Outcome::Failure((Status::Forbidden, err)),
            }
        } else {
            Outcome::Failure((Status::Forbidden, (AuthenticationError::NeedBasic)))
        }
    }
}

fn decode_basic_auth(auth: &str) -> Result<UserWithPlaintextPassword, AuthenticationError> {
    let base64_credentials = auth.replacen("Basic ", "", 1);
    match Basic::from_str(&base64_credentials) {
        Ok(basic) => {
            if basic.password.is_some() {
                Ok(UserWithPlaintextPassword {
                    user_name: basic.username,
                    plaintext_password: basic.password.unwrap(),
                })
            } else {
                Err(AuthenticationError::NoPassword)
            }
        }
        Err(_) => Err(AuthenticationError::IncorrectBase64),
    }
}

impl From<User> for UserWithoutPassword {
    fn from(user: User) -> Self {
        UserWithoutPassword {
            id: user.id,
            user_name: user.user_name,
        }
    }
}

// Get the user by name and verify that the password is correct
pub fn validate_user(
    user_to_validate: &UserWithPlaintextPassword,
    conn: &SqliteConnection,
) -> Result<UserWithoutPassword, AuthenticationError> {
    let u = users::table
        .filter(users::user_name.eq(&user_to_validate.user_name))
        .first::<User>(conn);

    match u {
        Ok(u) => match verify(&user_to_validate.plaintext_password, &u.hashed_password).unwrap() {
            true => Ok(UserWithoutPassword::from(u)),
            false => Err(AuthenticationError::IncorrectUsernameOrPassword),
        },
        Err(DieselError::NotFound) => Err(AuthenticationError::IncorrectUsernameOrPassword),
        Err(e) => Err(AuthenticationError::DieselError(e)),
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;

    #[test]
    fn decodes_basic_auth_user() {
        // mickjohn:qwerty
        let base64 = "bWlja2pvaG46cXdlcnR5";
        let expected = UserWithPlaintextPassword {
            user_name: "mickjohn".to_string(),
            plaintext_password: "qwerty".to_string(),
        };
        assert_eq!(decode_basic_auth(&base64).unwrap(), expected);
    }

    #[test]
    fn fails_for_incorrect_basic_auth() {
        let base64 = "complete gibberish";
        assert_eq!(
            decode_basic_auth(&base64),
            Err(AuthenticationError::IncorrectBase64)
        );
    }

    #[test]
    fn fails_when_theres_no_password() {
        // mickjohn
        let base64 = "bWlja2pvaG4=";
        assert_eq!(
            decode_basic_auth(&base64),
            Err(AuthenticationError::NoPassword)
        );
    }
}
