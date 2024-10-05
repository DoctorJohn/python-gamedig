use gamedig::{GDError, GDErrorKind};
use pyo3::{create_exception, exceptions::PyException, PyErr};

create_exception!(gamedig, GameDigError, PyException);
create_exception!(gamedig, PacketOverflowError, GameDigError);
create_exception!(gamedig, PacketUnderflowError, GameDigError);
create_exception!(gamedig, PacketBadError, GameDigError);
create_exception!(gamedig, PacketSendError, GameDigError);
create_exception!(gamedig, PacketReceiveError, GameDigError);
create_exception!(gamedig, DigDecompressError, GameDigError);
create_exception!(gamedig, DigSocketConnectError, GameDigError);
create_exception!(gamedig, SocketBindError, GameDigError);
create_exception!(gamedig, InvalidInputError, GameDigError);
create_exception!(gamedig, BadGameError, GameDigError);
create_exception!(gamedig, AutoQueryError, GameDigError);
create_exception!(gamedig, ProtocolFormatError, GameDigError);
create_exception!(gamedig, UnknownEnumCastError, GameDigError);
create_exception!(gamedig, JsonParseError, GameDigError);
create_exception!(gamedig, TypeParseError, GameDigError);
create_exception!(gamedig, HostLookupError, GameDigError);

pub fn gd_error_to_py_err(err: GDError) -> PyErr {
    match err.kind {
        GDErrorKind::PacketOverflow => PacketOverflowError::new_err(match err.source {
            None => "The received packet was bigger than the buffer size.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::PacketUnderflow => PacketUnderflowError::new_err(match err.source {
            None => "The received packet was shorter than the expected one.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::PacketBad => PacketBadError::new_err(match err.source {
            None => "The received packet is badly formatted.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::PacketSend => PacketSendError::new_err(match err.source {
            None => "Couldn't send the packet.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::PacketReceive => PacketReceiveError::new_err(match err.source {
            None => "Couldn't receive data.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::Decompress => DigDecompressError::new_err(match err.source {
            None => "Couldn't decompress data.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::SocketConnect => DigSocketConnectError::new_err(match err.source {
            None => "Couldn't create a socket connection.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::SocketBind => SocketBindError::new_err(match err.source {
            None => "Couldn't bind a socket.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::InvalidInput => InvalidInputError::new_err(match err.source {
            None => "Invalid input into the library".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::BadGame => BadGameError::new_err(match err.source {
            None => {
                "The server response indicated that it is a different game than the game queried."
                    .to_string()
            }
            Some(source) => source.to_string(),
        }),
        GDErrorKind::AutoQuery => AutoQueryError::new_err(match err.source {
            None => "None of the attempted protocols were successful.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::ProtocolFormat => ProtocolFormatError::new_err(match err.source {
            None => "A protocol-defined expected format was not met.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::UnknownEnumCast => UnknownEnumCastError::new_err(match err.source {
            None => "Couldn't cast a value to an enum.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::JsonParse => JsonParseError::new_err(match err.source {
            None => "Couldn't parse a json string.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::TypeParse => TypeParseError::new_err(match err.source {
            None => "Couldn't parse a value.".to_string(),
            Some(source) => source.to_string(),
        }),
        GDErrorKind::HostLookup => HostLookupError::new_err(match err.source {
            None => "Couldn't find the host specified.".to_string(),
            Some(source) => source.to_string(),
        }),
    }
}
