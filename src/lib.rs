mod errors;

use ::gamedig as rust_gamedig;
use errors::*;
use pyo3::{exceptions::PyValueError, prelude::*};
use serde_pyobject::to_pyobject;

#[pyfunction]
#[pyo3(signature = (game_id, address, port=None))]
fn query(py: Python, game_id: &str, address: &str, port: Option<u16>) -> PyResult<PyObject> {
    let game = match rust_gamedig::GAMES.get(game_id) {
        None => return Err(PyValueError::new_err(format!("Unknown game id: {game_id}"))),
        Some(game) => game,
    };

    let parsed_address = match address.parse() {
        Err(err) => return Err(PyValueError::new_err(format!("{err}"))),
        Ok(parsed_address) => parsed_address,
    };

    match rust_gamedig::query(game, &parsed_address, port) {
        Err(err) => return Err(gd_error_to_py_err(err)),
        Ok(response) => {
            let response_json = response.as_json();
            let py_response = to_pyobject(py, &response_json).unwrap();
            Ok(py_response.into_py(py))
        }
    }
}

#[pymodule]
fn gamedig(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("GameDigError", m.py().get_type_bound::<GameDigError>())?;
    m.add(
        "PacketOverflowError",
        m.py().get_type_bound::<PacketOverflowError>(),
    )?;
    m.add(
        "PacketUnderflowError",
        m.py().get_type_bound::<PacketUnderflowError>(),
    )?;
    m.add("PacketBadError", m.py().get_type_bound::<PacketBadError>())?;
    m.add(
        "PacketSendError",
        m.py().get_type_bound::<PacketSendError>(),
    )?;
    m.add(
        "PacketReceiveError",
        m.py().get_type_bound::<PacketReceiveError>(),
    )?;
    m.add(
        "DigDecompressError",
        m.py().get_type_bound::<DigDecompressError>(),
    )?;
    m.add(
        "DigSocketConnectError",
        m.py().get_type_bound::<DigSocketConnectError>(),
    )?;
    m.add(
        "SocketBindError",
        m.py().get_type_bound::<SocketBindError>(),
    )?;
    m.add(
        "InvalidInputError",
        m.py().get_type_bound::<InvalidInputError>(),
    )?;
    m.add("BadGameError", m.py().get_type_bound::<BadGameError>())?;
    m.add("AutoQueryError", m.py().get_type_bound::<AutoQueryError>())?;
    m.add(
        "ProtocolFormatError",
        m.py().get_type_bound::<ProtocolFormatError>(),
    )?;
    m.add(
        "UnknownEnumCastError",
        m.py().get_type_bound::<UnknownEnumCastError>(),
    )?;
    m.add("JsonParseError", m.py().get_type_bound::<JsonParseError>())?;
    m.add("TypeParseError", m.py().get_type_bound::<TypeParseError>())?;
    m.add(
        "HostLookupError",
        m.py().get_type_bound::<HostLookupError>(),
    )?;
    m.add_function(wrap_pyfunction!(query, m)?)?;
    Ok(())
}
