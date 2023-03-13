use super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: String) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }
    // convert to String from struct
    pub fn into_inner(self) -> String {
        self.0
    }
    // borrows the content
    pub fn as_str(&self) -> &str {
        &self.0.as_str()
    }
}
