/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   28 Jan 2022, 13:50:37
 * Last edited:
 *   28 Jan 2022, 15:24:46
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains common error types that span over multiple packages and/or
 *   modules.
**/

use std::path::PathBuf;


/***** ERROR ENUMS *****/
/// Errors that relate to finding Brane directories
#[derive(Debug, thiserror::Error)]
pub enum SystemDirectoryError {
    /// Could not find the user local data folder
    #[error("Could not find the user's local data directory for your OS (reported as {})", std::env::consts::OS)]
    UserLocalDataDirNotFound,
    /// Could not find the user config folder
    #[error("Could not find the user's config directory for your OS (reported as {})", std::env::consts::OS)]
    UserConfigDirNotFound,

    /// Could not find brane's folder in the data folder
    #[error("Brane data directory '{}' not found", path.display())]
    BraneLocalDataDirNotFound { path: PathBuf },
    /// Could not find brane's folder in the config folder
    #[error("Brane config directory '{}' not found", path.display())]
    BraneConfigDirNotFound { path: PathBuf },
}

/// Errors that relate to encoding or decoding output
#[derive(Debug, thiserror::Error)]
pub enum EncodeDecodeError {
    /// Could not decode the given string from Base64 binary data
    #[error("Could not decode string input as Base64")]
    Base64DecodeError { source: base64::DecodeError },

    /// Could not decode the given raw binary using UTF-8
    #[error("Could not decode binary input as UTF-8")]
    Utf8DecodeError { source: std::string::FromUtf8Error },

    /// Could not decode the given input as JSON
    #[error("Could not decode string input as JSON")]
    JsonDecodeError { source: serde_json::Error },
}
