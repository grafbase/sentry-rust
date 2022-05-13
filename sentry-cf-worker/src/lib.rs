use sentry_core::{constants::USER_AGENT, IntoDsn};
use worker::{console_error, wasm_bindgen_futures};

pub use sentry_core::protocol::*;
pub use sentry_core::Level;

static SENTRY_AUTH_HEADER: &str = "X-Sentry-Auth";

pub fn send_envelope(sentry_ingest_url: &str, envelope: Envelope) {
    // FIXME
    let dsn = sentry_ingest_url.clone().into_dsn().unwrap().unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let auth = dsn.to_auth(Some(&USER_AGENT)).to_string();
        let url = dsn.envelope_api_url().to_string();
        let mut body = Vec::new();

        match envelope.to_writer(&mut body) {
            Ok(_) => {}
            Err(error) => console_error!("Error writing envelope: {}", error),
        }

        let request = surf::Client::new()
            .post(&url)
            .header(SENTRY_AUTH_HEADER, &auth)
            .body(body)
            .await;

        match request {
            Ok(_) => {}
            Err(error) => console_error!("Request error: {}", error),
        }
    });
}
