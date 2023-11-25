#![cfg(feature = "ssr")]

use {
    anyhow::{Context, Result},
    sentry::types::Dsn,
    serde::{Deserialize, Deserializer},
    std::{
        fs::File,
        io::BufReader,
        net::SocketAddr,
        num::NonZeroU64,
        path::{Path, PathBuf},
    },
    tracing_subscriber::EnvFilter,
};

pub type TlsConfig = rustls::ServerConfig;

/// The configuration.
// Doesn't derive Debug to avoid leaking secrets
#[derive(Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,

    /// Sets the TLS configuration.
    ///
    /// If provided, listens on port 443 with TLS and redirects port http
    /// requests on port 80 to 443.
    #[serde(deserialize_with = "parse_tls_config")]
    pub tls: Option<TlsConfig>,

    /// Sets the socket address to listen on for incoming connections.
    /// Does not use TLS.
    pub debug_listening_address: Option<SocketAddr>,

    /// The path to the assets directory. This directory is served at the root.
    pub assets: String,

    /// Defines a timeout for reading client request head.
    /// If a client does not transmit the entire set headers within this time,
    /// the request is terminated with a 408 (Request Timeout) error.
    pub shutdown_timeout_seconds: NonZeroU64,

    /// The healthchecks.io success ping url.
    pub health_check_ping_url: Option<String>,
}

/// Logging filters and sentry reporting config.
#[derive(Deserialize)]
pub struct LoggingConfig {
    /// Logging directives; acceptable directive must must follow [this][1]
    /// format.
    ///
    /// [1]: https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
    #[serde(deserialize_with = "parse_env_filter")]
    pub directives: EnvFilter,

    /// The sentry dsn.
    pub sentry_data_source_name: Option<Dsn>,
}

/// The paths to the tls certificate and private key.
#[derive(Deserialize)]
pub struct TlsCertPaths {
    /// The filepath to the tls certificate.
    pub cert: PathBuf,

    /// The filepath to the tls private key.
    pub cert_key: PathBuf,
}

/// Read the config from the file at CONFIG and parse it.
pub fn parse_from_env() -> Result<Config> {
    let path = std::env::var("CONFIG").context("CONFIG is not set")?;
    let file = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read file at {path:?}"))?;
    ron::from_str(&file).context("failed to parse config file")
}

fn parse_env_filter<'de, D>(deserializer: D) -> Result<EnvFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let directives: String = Deserialize::deserialize(deserializer)?;
    EnvFilter::builder()
        .parse(directives)
        .map_err(serde::de::Error::custom)
}

fn parse_tls_config<'de, D>(deserializer: D) -> Result<Option<TlsConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let tls: Option<TlsCertPaths> = Deserialize::deserialize(deserializer)?;
    tls.map(|tls| create_tls(&tls.cert, &tls.cert_key).map_err(serde::de::Error::custom))
        .transpose()
}

fn create_tls(cert: &Path, cert_key: &Path) -> Result<TlsConfig> {
    let open = |path| File::open(path).with_context(|| format!("error opening {path:?}"));
    let cert_chain = rustls_pemfile::certs(&mut BufReader::new(open(cert)?))
        .context("couldn't parse cert")?
        .into_iter()
        .map(rustls::Certificate)
        .collect();
    let key = {
        let mut keys = rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(open(cert_key)?))
            .context("couldn't parse cert key")?;
        anyhow::ensure!(!keys.is_empty(), "no cert key found");
        rustls::PrivateKey(keys.swap_remove(0))
    };
    TlsConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .context("couldn't create certificate chain")
}
