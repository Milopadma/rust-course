pub mod field;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub name: field::ClipID,
    pub shortcode: field,
    pub content: field,
    pub title: field,
    pub posted: field,
    pub expires: field,
    pub password: field,
    pub hits: field,
}
