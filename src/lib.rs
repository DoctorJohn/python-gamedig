mod errors;
mod query;

use crate::errors::*;
use pyo3::prelude::*;

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
    m.add_function(wrap_pyfunction!(crate::query::query, m)?)?;
    Ok(())
}
