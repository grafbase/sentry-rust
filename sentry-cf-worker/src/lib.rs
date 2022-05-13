use sentry_core::{constants::USER_AGENT, IntoDsn};
use thiserror::Error;
use worker::{console_error, wasm_bindgen_futures};

pub use sentry_core::protocol::*;
pub use sentry_core::Level;

static SENTRY_AUTH_HEADER: &str = "X-Sentry-Auth";

#[derive(Error)]
pub enum SentryError {
    #[error("invalid sentry ingest url")]
    InvalidUrl,
    #[error("the call to sentry returned an error")]
    Request(surf::Error),
    #[error("could not write the given envelope")]
    WriteEnvelope,
}

pub async fn send_envelope(sentry_ingest_url: &str, envelope: Envelope) -> Result<(), SentryError> {
    let dsn = sentry_ingest_url
        .clone()
        .into_dsn()
        .map_err(|_| SentryError::InvalidUrl)?
        .unwrap();

    let auth = dsn.to_auth(Some(&USER_AGENT)).to_string();
    let url = dsn.envelope_api_url().to_string();
    let mut body = Vec::new();

    envelope
        .to_writer(&mut body)
        .map_err(|_| SentryError::WriteEnvelope)?;

    let _request = surf::Client::new()
        .post(&url)
        .header(SENTRY_AUTH_HEADER, &auth)
        .body(body)
        .await
        .map_err(|error| SentryError::Request(error));

    Ok(())
}
