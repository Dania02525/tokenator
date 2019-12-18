use diesel::{self, prelude::*};
use crate::models::{Token, Credentials};
use crate::errors::NotFoundError;

use crate::DbConn;

use ring::{digest, pbkdf2};
use crate::byteorder::ByteOrder;
use byteorder::BigEndian;

pub fn validate_credentials(credentials: Credentials, conn: DbConn) -> Result<Token, NotFoundError> {
  use crate::schema::users::dsl::*;

  let user_name: String = credentials.user_name;
  let password: String = credentials.password;

  let raw_hash = users
    .select(encrypted_password)
    .filter(email.eq(user_name))
    .first::<String>(&conn.0)?;

  let parts: Vec<&str> = raw_hash.split(":").collect();
  let salt = hex::decode(parts[1])?;
  let hash = hex::decode(parts[2])?;
  let _rounds = BigEndian::read_u32(parts[0].as_bytes());

  // hardcoded rounds in here since it was locking the verify
  pbkdf2::verify(&digest::SHA1, 1000, &salt, password.as_bytes(), &hash)?;

  Ok(create_oauth_token())
}

fn create_oauth_token() -> Token {
  Token {
    created_at: String::from("string"),
    modified_at: String::from("string"),
    token_id: String::from("string"),
    expires_at: String::from("string"),
    scope: String::from("string"),
    token_type: String::from("string"),
    mac_key: String::from("string"),
    mac_algorithm: String::from("string"),
    client_id: String::from("string"),
    grant_type: String::from("string"),
    recurly_admin: false,
    time_delta: String::from("string"),
    updated_at: String::from("string")
  }
}
