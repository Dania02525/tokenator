#[derive(Serialize, Queryable)]
pub struct User {
  pub user_id: i64,
  pub email: String,
  pub encrypted_password: String
}

#[derive(Deserialize)]
pub struct Credentials {
  pub grant_type: String,
  pub user_name: String,
  pub password: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
  pub created_at: String,
  pub modified_at: String,
  pub token_id: String,
  pub expires_at: String,
  pub scope: String,
  pub token_type: String,
  pub mac_key: String,
  pub mac_algorithm: String,
  pub client_id: String,
  pub grant_type: String,
  pub recurly_admin: bool,
  pub time_delta: String,
  pub updated_at: String
}


