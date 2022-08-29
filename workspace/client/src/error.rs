use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Error generated when a path is not a file.
    #[error("path {0} is not a file")]
    NotFile(PathBuf),

    /// Error generated when a path is not a directory.
    #[error("path {0} is not a directory")]
    NotDirectory(PathBuf),

    /// Error generated when a file already exists.
    #[error("file already exists {0}")]
    FileExists(PathBuf),

    /// Error generated when passwords do not match.
    #[error("passwords do not match, try again")]
    PasswordMismatch,

    /// Error generated on unexpected HTTP response code.
    #[error("unexpected response code {0}")]
    ResponseCode(u16),

    /// Error generated by the io module.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Error generated by the HTTP client library.
    #[error(transparent)]
    Request(#[from] reqwest::Error),

    /// Error generated by the URL library.
    #[error(transparent)]
    Url(#[from] url::ParseError),

    /// Error generated by the keystore library.
    #[error(transparent)]
    Keystore(#[from] web3_keystore::KeyStoreError),

    /// Error generated by the JSON library.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Error generate by the ECDSA library.
    #[error(transparent)]
    Ecdsa(#[from] k256::ecdsa::Error),

    #[error(transparent)]
    Readline(#[from] rustyline::error::ReadlineError),
}
