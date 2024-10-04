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
        GDErrorKind::PacketOverflow => {
            PacketOverflowError::new_err("The received packet was bigger than the buffer size.")
        }
        GDErrorKind::PacketUnderflow => {
            PacketUnderflowError::new_err("The received packet was shorter than the expected one.")
        }
        GDErrorKind::PacketBad => {
            PacketBadError::new_err("The received packet is badly formatted.")
        }
        GDErrorKind::PacketSend => PacketSendError::new_err("Couldn't send the packet."),
        GDErrorKind::PacketReceive => PacketReceiveError::new_err("Couldn't receive data."),
        GDErrorKind::Decompress => DigDecompressError::new_err("Couldn't decompress data."),
        GDErrorKind::SocketConnect => {
            DigSocketConnectError::new_err("Couldn't create a socket connection.")
        }
        GDErrorKind::SocketBind => SocketBindError::new_err("Couldn't bind a socket."),
        GDErrorKind::InvalidInput => InvalidInputError::new_err("Invalid input to the library."),
        GDErrorKind::BadGame => BadGameError::new_err(
            "The server response indicated that it is a different game than the game queried.",
        ),
        GDErrorKind::AutoQuery => {
            AutoQueryError::new_err("None of the attempted protocols were successful.")
        }
        GDErrorKind::ProtocolFormat => {
            ProtocolFormatError::new_err("A protocol-defined expected format was not met.")
        }
        GDErrorKind::UnknownEnumCast => {
            UnknownEnumCastError::new_err("Couldn't cast a value to an enum.")
        }
        GDErrorKind::JsonParse => JsonParseError::new_err("Couldn't parse a json string."),
        GDErrorKind::TypeParse => TypeParseError::new_err("Couldn't parse a value."),
        GDErrorKind::HostLookup => HostLookupError::new_err("Couldn't find the host specified."),
    }
}
