mod errors;
mod query;

use crate::errors::*;
use pyo3::prelude::*;

#[pymodule]
fn gamedig(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("GameDigError", m.py().get_type::<GameDigError>())?;
    m.add(
        "PacketOverflowError",
        m.py().get_type::<PacketOverflowError>(),
    )?;
    m.add(
        "PacketUnderflowError",
        m.py().get_type::<PacketUnderflowError>(),
    )?;
    m.add("PacketBadError", m.py().get_type::<PacketBadError>())?;
    m.add("PacketSendError", m.py().get_type::<PacketSendError>())?;
    m.add(
        "PacketReceiveError",
        m.py().get_type::<PacketReceiveError>(),
    )?;
    m.add(
        "DigDecompressError",
        m.py().get_type::<DigDecompressError>(),
    )?;
    m.add(
        "DigSocketConnectError",
        m.py().get_type::<DigSocketConnectError>(),
    )?;
    m.add("SocketBindError", m.py().get_type::<SocketBindError>())?;
    m.add("InvalidInputError", m.py().get_type::<InvalidInputError>())?;
    m.add("BadGameError", m.py().get_type::<BadGameError>())?;
    m.add("AutoQueryError", m.py().get_type::<AutoQueryError>())?;
    m.add(
        "ProtocolFormatError",
        m.py().get_type::<ProtocolFormatError>(),
    )?;
    m.add(
        "UnknownEnumCastError",
        m.py().get_type::<UnknownEnumCastError>(),
    )?;
    m.add("JsonParseError", m.py().get_type::<JsonParseError>())?;
    m.add("TypeParseError", m.py().get_type::<TypeParseError>())?;
    m.add("HostLookupError", m.py().get_type::<HostLookupError>())?;
    m.add_function(wrap_pyfunction!(crate::query::query, m)?)?;
    Ok(())
}
