use ::gamedig as rust_gamedig;
use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    prelude::*,
};
use serde_pyobject::to_pyobject;

create_exception!(gamedig, GameDigError, PyException);

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
        Err(err) => return Err(GameDigError::new_err(format!("{:?}", err.kind))),
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
    m.add_function(wrap_pyfunction!(query, m)?)?;
    Ok(())
}
