pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ClipError {
    #[error("Invalid passwrod: {0}")]
    InvalidPassword(String),
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    #[error("Empty Content")]
    EmptyContent,
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Date parse error: {0}")]
    DataParse(#[from] chrono::ParseError),
    #[error("Invalid id: {0}")]
    Id(#[from] uuid::Error),
    #[error("Parse int error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub name: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
