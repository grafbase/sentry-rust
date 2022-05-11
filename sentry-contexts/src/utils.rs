use sentry_core::protocol::{Context, DeviceContext, Map, OsContext, RuntimeContext};

include!(concat!(env!("OUT_DIR"), "/constants.gen.rs"));

mod model_support {
    pub fn get_model() -> Option<String> {
        None
    }

    pub fn get_family() -> Option<String> {
        None
    }
}

/// Returns the server name (hostname) if available.
pub fn server_name() -> Option<String> {
    Some("WASM".to_owned())
}

/// Returns the OS context
pub fn os_context() -> Option<Context> {
    Some(
        OsContext {
            name: Some("WASM".into()),
            ..Default::default()
        }
        .into(),
    )
}

/// Returns the rust info.
pub fn rust_context() -> Context {
    RuntimeContext {
        name: Some("rustc".into()),
        version: RUSTC_VERSION.map(|x| x.into()),
        other: {
            let mut map = Map::default();
            if let Some(channel) = RUSTC_CHANNEL {
                map.insert("channel".to_string(), channel.into());
            }
            map
        },
    }
    .into()
}

/// Returns the device context.
pub fn device_context() -> Context {
    DeviceContext {
        model: model_support::get_model(),
        family: model_support::get_family(),
        arch: Some(ARCH.into()),
        ..Default::default()
    }
    .into()
}
