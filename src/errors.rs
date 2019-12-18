use std::error::Error;
use std::fmt;


#[derive(Debug, Serialize)]
pub struct NotFoundError {
  pub message: String
}

impl NotFoundError {
    pub fn new(msg: &str) -> NotFoundError {
        NotFoundError{message: msg.to_string()}
    }
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl Error for NotFoundError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<diesel::result::Error> for NotFoundError {
  fn from(_err: diesel::result::Error) -> Self {
    NotFoundError::new("User or password not found")
  }
}

// TODO: obscure these messages, so we don't expose internal workings
impl From<ring::error::Unspecified> for NotFoundError {
  fn from(_err: ring::error::Unspecified) -> Self {
    NotFoundError::new("User or password not found: hash not matched")
  }
}


impl From<hex::FromHexError> for NotFoundError {
  fn from(_err: hex::FromHexError) -> Self {
    NotFoundError::new("User or password not found: hash in bad format")
  }
}
