use font_kit::error::{SelectionError, FontLoadingError};
use png::DecodingError;
use tauri::InvokeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Raqote PNG encoding error")]
    PngEncoding,

    #[error("PNG decoding error")]
    PngDecoding(#[from] DecodingError),

    #[error("Font selection error (do you have Comic Sans?)")]
    FontSelectionError(#[from] SelectionError),

    #[error("Font loading error")]
    FontLoadingError(#[from] FontLoadingError),
}

impl From<Error> for InvokeError {
    fn from(err: Error) -> Self {
        InvokeError::from_anyhow(err.into())
    }
}
