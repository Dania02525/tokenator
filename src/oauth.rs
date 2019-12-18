use rocket_contrib::json::Json;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Credentials {
    grant_type: String,
    user_name: String,
    password: String
}

#[derive(Serialize)]
pub struct Token {
  createdAt: String,
  modifiedAt: String,
  tokenId: String,
  expiresAt: String,
  scope: String,
  tokenType: String,
  macKey: String,
  macAlgorithm: String,
  clientId: String,
  grantType: String,
  recurlyAdmin: bool,
  timeDelta: String,
  updatedAt: String
}

#[post("/oauth/tokens", data = "<credentials>")]
pub fn create(credentials: Json<Credentials>) -> Json<Token> {
  Json(createOauthToken(credentials.into_inner()))
}

fn createOauthToken(credentials: Credentials) -> Token {
  Token {
    createdAt: String::from("string"),
    modifiedAt: String::from("string"),
    tokenId: String::from("string"),
    expiresAt: String::from("string"),
    scope: String::from("string"),
    tokenType: String::from("string"),
    macKey: String::from("string"),
    macAlgorithm: String::from("string"),
    clientId: String::from("string"),
    grantType: String::from("string"),
    recurlyAdmin: false,
    timeDelta: String::from("string"),
    updatedAt: String::from("string")
  }
}
