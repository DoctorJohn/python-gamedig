use crate::errors::*;
use gamedig::{self as rust_gamedig, TimeoutSettings};
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
) -> PyResult<PyObject> {
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
            let connect_timeout = timeout_settings
                .get("connect")
                .map(|&x| std::time::Duration::from_secs(x as u64));
            let read_timeout = timeout_settings
                .get("read")
                .map(|&x| std::time::Duration::from_secs(x as u64));
            let write_timeout = timeout_settings
                .get("write")
                .map(|&x| std::time::Duration::from_secs(x as u64));
            let retries = timeout_settings
                .get("retries")
                .map(|&x| x as usize)
                .unwrap();

            match TimeoutSettings::new(read_timeout, write_timeout, connect_timeout, retries) {
                Err(err) => return Err(gd_error_to_py_err(err)),
                Ok(parsed_timeout_settings) => Some(parsed_timeout_settings),
            }
        }
    };

    let parsed_extra_settings = match extra_settings {
        None => None,
        Some(extra_settings) => match from_pyobject(extra_settings.clone()) {
            Ok(parsed) => Some(parsed),
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
