#[derive(Debug)]
pub enum ClientError {
    /// Http error
    Http(reqwest::Error),

    /// Authorization error
    Authorization,

    /// Json parsing error
    Json(serde_json::Error),
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::Http(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::Json(err)
    }
}