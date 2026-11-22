use crate::errors::*;
use gamedig::{
    self as rust_gamedig, protocols::Protocol, ExtraRequestSettings, Game, TimeoutSettings,
};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};
use serde_pyobject::{from_pyobject, to_pyobject};
use std::collections::HashMap;

#[pyfunction]
#[pyo3(signature = (game_id, address, port=None, timeout_settings=None, extra_settings=None))]
pub fn query(
    py: Python,
    game_id: &str,
    address: &str,
    port: Option<u16>,
    timeout_settings: Option<HashMap<String, u16>>,
    extra_settings: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let game = match rust_gamedig::GAMES.get(game_id) {
        None => return Err(PyValueError::new_err(format!("Unknown game id: {game_id}"))),
        Some(game) => game,
    };

    let parsed_address = match address.parse() {
        Err(err) => return Err(PyValueError::new_err(format!("{err}"))),
        Ok(parsed_address) => parsed_address,
    };

    let parsed_timeout_settings = match timeout_settings {
        None => None,
        Some(timeout_settings) => {
            // Omitted settings must not be passed as None, gamedig would block indefinitely
            let defaults = TimeoutSettings::default();
            let connect_timeout = timeout_settings
                .get("connect")
                .map(|&x| std::time::Duration::from_secs(x as u64))
                .or(defaults.get_connect());
            let read_timeout = timeout_settings
                .get("read")
                .map(|&x| std::time::Duration::from_secs(x as u64))
                .or(defaults.get_read());
            let write_timeout = timeout_settings
                .get("write")
                .map(|&x| std::time::Duration::from_secs(x as u64))
                .or(defaults.get_write());
            let retries = timeout_settings
                .get("retries")
                .map(|&x| x as usize)
                .unwrap_or(defaults.get_retries());

            match TimeoutSettings::new(read_timeout, write_timeout, connect_timeout, retries) {
                Err(err) => return Err(gd_error_to_py_err(err)),
                Ok(parsed_timeout_settings) => Some(parsed_timeout_settings),
            }
        }
    };

    let parsed_extra_settings = match extra_settings {
        None => None,
        Some(extra_settings) => match from_pyobject(extra_settings.clone()) {
            Ok(parsed) => Some(with_game_defaults(parsed, game)),
            Err(err) => return Err(err.into()),
        },
    };

    match rust_gamedig::query_with_timeout_and_extra_settings(
        game,
        &parsed_address,
        port,
        parsed_timeout_settings,
        parsed_extra_settings,
    ) {
        Err(err) => return Err(gd_error_to_py_err(err)),
        Ok(response) => {
            let response_json = response.as_json();
            let py_response = to_pyobject(py, &response_json).unwrap();
            Ok(py_response.into())
        }
    }
}

/// Fills in the settings omitted by the user with the game's own defaults.
/// gamedig would otherwise ignore the game's defaults as soon as any settings are passed.
fn with_game_defaults(settings: ExtraRequestSettings, game: &Game) -> ExtraRequestSettings {
    // gamedig only uses game defaults for Valve games, other protocols have their own defaults
    if !matches!(game.protocol, Protocol::Valve(_)) {
        return settings;
    }

    let defaults = &game.request_settings;
    ExtraRequestSettings {
        hostname: settings.hostname.or_else(|| defaults.hostname.clone()),
        protocol_version: settings.protocol_version.or(defaults.protocol_version),
        gather_players: settings.gather_players.or(defaults.gather_players),
        gather_rules: settings.gather_rules.or(defaults.gather_rules),
        check_app_id: settings.check_app_id.or(defaults.check_app_id),
    }
}
