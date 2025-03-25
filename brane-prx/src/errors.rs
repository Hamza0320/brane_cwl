//  ERRORS.rs
//    by Lut99
//
//  Created:
//    23 Nov 2022, 11:43:56
//  Last edited:
//    03 Jan 2024, 14:55:04
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the errors that may occur in the `brane-prx` crate.
//

use std::net::SocketAddr;
use std::ops::RangeInclusive;

use reqwest::StatusCode;
use specifications::address::Address;
use url::Url;


/***** LIBRARY *****/
/// Defines errors that relate to redirection.
#[derive(Debug, thiserror::Error)]
pub enum RedirectError {
    /// No domain name given in the given URL
    #[error("No domain name found in '{raw}'")]
    NoDomainName { raw: String },
    /// The given URL is not a valid URL
    #[error("Failed to parse '{raw}' as a valid URL")]
    IllegalUrl { raw: String, source: url::ParseError },
    /// Asked to do TLS with an IP
    #[error("Got a request for TLS but with a non-hostname {kind} address provided")]
    TlsWithNonHostnameError { kind: String },
    /// The given hostname was illegal
    #[error("Cannot parse '{raw}' as a valid server name")]
    IllegalServerName { raw: String, source: rustls::client::InvalidDnsNameError },
    /// Failed to create a new tcp listener.
    #[error("Failed to create new TCP listener on '{address}'")]
    ListenerCreateError { address: SocketAddr, source: std::io::Error },
    /// Failed to create a new socks5 client.
    #[error("Failed to create new SOCKS5 client to '{address}'")]
    Socks5CreateError { address: Address, source: anyhow::Error },
    /// Failed to create a new socks6 client.
    #[error("Failed to create new SOCKS6 client to '{address}'")]
    Socks6CreateError { address: Address, source: anyhow::Error },

    /// Failed to connect using a regular ol' TcpStream.
    #[error("Failed to connect to '{address}'")]
    TcpStreamConnectError { address: String, source: std::io::Error },
    /// Failed to connect using a SOCKS5 client.
    #[error("Failed to connect to '{address}' through SOCKS5-proxy '{proxy}'")]
    Socks5ConnectError { address: String, proxy: Address, source: anyhow::Error },
    /// Failed to connect using a SOCKS6 client.
    #[error("Failed to connect to '{address}' through SOCKS6-proxy '{proxy}'")]
    Socks6ConnectError { address: String, proxy: Address, source: anyhow::Error },

    /// The given port for an incoming path is in the outgoing path's range.
    #[error("Given port '{}' is within range {}-{} of the outgoing connection ports; please choose another (or choose another outgoing port range)", port, range.start(), range.end())]
    PortInOutgoingRange { port: u16, range: RangeInclusive<u16> },
}


/// Defines errors for clients of the proxy.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// The given URL was not a URL
    #[error("'{raw}' is not a valid URL")]
    IllegalUrl { raw: String, source: url::ParseError },
    /// Failed to update the given URL with a new scheme.
    #[error("Failed to update '{url}' with new scheme '{scheme}'")]
    UrlSchemeUpdateError { url: Url, scheme: String },
    /// Failed to update the given URL with a new host.
    #[error("Failed to update '{url}' with new host '{host}'")]
    UrlHostUpdateError { url: Url, host: String, source: url::ParseError },
    /// Failed to update the given URL with a new port.
    #[error("Failed to update '{url}' with new port '{port}'")]
    UrlPortUpdateError { url: Url, port: u16 },

    /// Failed to build a request.
    #[error("Failed to build a request to '{address}'")]
    RequestBuildError { address: String, source: reqwest::Error },
    /// Failed to send a request on its way.
    #[error("Failed to send request to '{address}'")]
    RequestError { address: String, source: reqwest::Error },
    /// The request failed with a non-success status code.
    #[error("Request to '{}' failed with status code {} ({}){}", address, code.as_u16(), code.canonical_reason().unwrap_or("??"), if let Some(err) = err { format!(": {err}") } else { String::new() })]
    RequestFailure { address: String, code: StatusCode, err: Option<String> },
    /// Failed to get the body of a response as some text.
    #[error("Failed to get body of response from '{address}' as plain text")]
    RequestTextError { address: String, source: reqwest::Error },
    /// Failed to parse the response's body as a port number.
    #[error("Failed to parse '{raw}' received from '{address}' as a port number")]
    RequestPortParseError { address: String, raw: String, source: std::num::ParseIntError },
}
