use std::io;

use image::ImageError;
use tauri::InvokeError;
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Image error")]
    ImageError(#[from] ImageError),
}

impl From<Error> for InvokeError {
    fn from(err: Error) -> Self {
        InvokeError::from_anyhow(err.into())
    }
}
