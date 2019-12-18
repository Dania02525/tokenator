use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{User, Token, Credentials};

use crate::DbConn;

use crate::errors::NotFoundError;

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[post("/oauth/tokens", data = "<credentials>")]
pub fn create_token(credentials: Json<Credentials>, conn: DbConn) -> Result<Json<Token>, Json<NotFoundError>> {
  let result = crate::utils::validate_credentials(credentials.into_inner(), conn);

  match result {
    Ok(result) => Ok(Json(result)),
    Err(result) => Err(Json(result))
  }
}

#[get("/list_users")]
pub fn list_users(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::users::dsl::*;

    users.load(&conn.0).map_err(|err| -> String {
        println!("Error querying users: {:?}", err);
        "Error querying users from the database".into()
    }).map(Json)
}
