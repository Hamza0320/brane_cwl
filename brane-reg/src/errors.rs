//  ERRORS.rs
//    by Lut99
//
//  Created:
//    26 Sep 2022, 15:13:34
//  Last edited:
//    16 Jan 2024, 17:27:57
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the errors that may occur in the `brane-reg` crate.
//

use std::net::SocketAddr;
use std::path::PathBuf;

/***** LIBRARY *****/
/// Defines Store-related errors.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Failed to parse from the given reader.
    #[error("Failed to parse the given store reader as YAML")]
    ReaderParseError { source: serde_yaml::Error },

    /// Failed to open the store file.
    #[error("Failed to open store file '{}'", path.display())]
    FileOpenError { path: PathBuf, source: std::io::Error },
    /// Failed to parse the store file.
    #[error("Failed to parse store file '{}' as YAML", path.display())]
    FileParseError { path: PathBuf, source: serde_yaml::Error },

    /// Failed to read the given directory.
    #[error("Failed to read directory '{}'", path.display())]
    DirReadError { path: PathBuf, source: std::io::Error },
    /// Failed to read an entry in the given directory.
    #[error("Failed to read entry {} in directory '{}'", i, path.display())]
    DirReadEntryError { path: PathBuf, i: usize, source: std::io::Error },
    /// Failed to read the AssetInfo file.
    #[error("Failed to load asset info file '{}'", path.display())]
    AssetInfoReadError { path: PathBuf, source: specifications::data::AssetInfoError },
}

/// Errors that relate to the customized serving process of warp.
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    /// Failed to create a new TcpListener and bind it to the given address.
    #[error("Failed to bind new TCP server to '{address}'")]
    ServerBindError { address: SocketAddr, source: std::io::Error },
    /// Failed to load the keypair.
    #[error("Failed to load keypair")]
    KeypairLoadError { source: brane_cfg::certs::Error },
    /// Failed to load the certificate root store.
    #[error("Failed to load root store")]
    StoreLoadError { source: brane_cfg::certs::Error },
    /// Failed to create a new ServerConfig for the TLS setup.
    #[error("Failed to create new TLS server configuration")]
    ServerConfigError { source: rustls::Error },
}

/// Errors that relate to the `/data` path (and nested).
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    /// Failed to serialize the contents of the store file (i.e., all known datasets)
    #[error("Failed to serialize known datasets")]
    StoreSerializeError { source: serde_json::Error },
    /// Failed to serialize the contents of a single dataset.
    #[error("Failed to serialize dataset metadata for dataset '{name}'")]
    AssetSerializeError { name: String, source: serde_json::Error },

    /// Failed to create a temporary directory.
    #[error("Failed to create a temporary directory")]
    TempDirCreateError { source: std::io::Error },
    /// Failed to archive the given dataset.
    #[error("Failed to archive data")]
    DataArchiveError { source: brane_shr::fs::Error },
    /// Failed to re-open the tar file after compressing.
    #[error("Failed to re-open tarball file '{}'", path.display())]
    TarOpenError { path: PathBuf, source: std::io::Error },
    /// Failed to read from the tar file.
    #[error("Failed to read from tarball file '{}'", path.display())]
    TarReadError { path: PathBuf, source: std::io::Error },
    /// Failed to send chunk of bytes on the body.
    #[error("Failed to send chunk of tarball file as body")]
    TarSendError { source: warp::hyper::Error },
    /// The given file was not a file, nor a directory.
    #[error("Dataset file '{}' is neither a file, nor a directory; don't know what to do with it", path.display())]
    UnknownFileTypeError { path: PathBuf },
    /// The given data path does not point to a data set, curiously enough.
    #[error("The data of dataset '{}' should be at '{}', but doesn't exist", name, path.display())]
    MissingData { name: String, path: PathBuf },
    /// The given result does not point to a data set, curiously enough.
    #[error("The data of intermediate result '{}' should be at '{}', but doesn't exist", name, path.display())]
    MissingResult { name: String, path: PathBuf },
}

impl warp::reject::Reject for DataError {}

/// Errors that relate to checker authorization.
#[derive(Debug, thiserror::Error)]
pub enum AuthorizeError {
    /// The client did not provide us with a certificate.
    #[error("No certificate provided")]
    ClientNoCert,

    /// Failed to load the policy file.
    #[error("Failed to load policy file")]
    PolicyFileError { source: brane_cfg::policies::Error },
    /// No policy matched this user/data pair.
    #[error("No matching policy rule found for user '{user}' / data '{data}' (did you forget a final AllowAll/DenyAll?)")]
    NoUserPolicy { user: String, data: String },
}
