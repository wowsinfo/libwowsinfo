//! App helpers (api key, outcome mapping, KV commands).

use crux_http::{HttpError, Response};
use crux_kv::KeyValueError;

use crate::APP_KEY;
use super::*;

pub(crate) const MISSING_KEY_MESSAGE: &str =
    "Missing Wargaming API key. Add one to keys.toml or set WOWSINFO_APP_KEY.";

pub(crate) fn api_key(config: &Config) -> String {
    if config.api_key.is_empty() {
        APP_KEY.to_string()
    } else {
        config.api_key.clone()
    }
}

pub(crate) fn http_outcome(result: Result<Response<String>, HttpError>) -> HttpOutcome {
    match result {
        Ok(response) => HttpOutcome::Ok {
            body: response.body().cloned().unwrap_or_default(),
        },
        Err(error) => HttpOutcome::Err {
            message: error.to_string(),
        },
    }
}

pub(crate) fn kv_outcome(result: Result<Option<Vec<u8>>, KeyValueError>) -> KvOutcome {
    match result {
        Ok(value) => KvOutcome::Ok {
            value: value.map(|bytes| String::from_utf8_lossy(&bytes).into_owned()),
        },
        Err(error) => KvOutcome::Err {
            message: error.to_string(),
        },
    }
}

pub(crate) fn kv_get_event(key: &'static str) -> Command<Effect, Event> {
    KeyValueCap::get(key).then_send(|result| Event::KvLoaded {
        key: key.to_string(),
        value: kv_outcome(result),
    })
}

pub(crate) fn kv_set_event(key: &'static str, value: String) -> Command<Effect, Event> {
    KeyValueCap::set(key, value.into_bytes()).then_send(|_| Event::ServerSaved)
}


