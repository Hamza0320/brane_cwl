//  ERRORS.rs
//    by Lut99
//
//  Created:
//    30 Nov 2022, 18:08:54
//  Last edited:
//    16 Jan 2024, 17:23:17
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines any errors that occur in the `brane-job` crate.
//

use std::path::PathBuf;


/***** LIBRARY *****/
/// Defines errors that relate to the ContainerHashes file.
#[derive(Debug, thiserror::Error)]
pub enum ContainerHashesError {
    /// Failed to read the given hash file.
    #[error("Failed to read hash file '{}'", path.display())]
    ReadError { path: PathBuf, source: std::io::Error },
    /// Failed to parse the given hash file as the appropriate YAML.
    #[error("Failed to parse hash file '{}' as YAML", path.display())]
    ParseError { path: PathBuf, source: serde_yaml::Error },
    /// There was a duplicate hash in there.
    #[error("Hash file '{}' contains duplicate hash '{}'", path.display(), hash)]
    DuplicateHash { path: PathBuf, hash: String },

    /// Failed to serialize the hash file.
    #[error("Failed to serialize hash file")]
    SerializeError { source: serde_yaml::Error },
    /// Failed to write to the given file.
    #[error("Failed to write hash file to '{}'", path.display())]
    WriteError { path: PathBuf, source: std::io::Error },
}
