from typing import Any, Dict, Optional, TypedDict, NotRequired, TypeAlias, Literal

class TimeoutSettings(TypedDict):
    retries: int
    read: NotRequired[int]
    write: NotRequired[int]
    connect: NotRequired[int]

GatherToggle: TypeAlias = Literal["Skip", "Try", "Enforce"]

class ExtraRequestSettings(TypedDict):
    hostname: NotRequired[str]
    protocol_version: NotRequired[int]
    gather_players: NotRequired[GatherToggle]
    gather_rules: NotRequired[GatherToggle]
    check_app_id: NotRequired[bool]

def query(
    game_id: str,
    address: str,
    port: Optional[int] = None,
    timeout_settings: Optional[TimeoutSettings] = None,
    extra_settings: Optional[ExtraRequestSettings] = None,
) -> Dict[str, Any]: ...

class GameDigError(Exception): ...
class PacketOverflowError(GameDigError): ...
class PacketUnderflowError(GameDigError): ...
class PacketBadError(GameDigError): ...
class PacketSendError(GameDigError): ...
class PacketReceiveError(GameDigError): ...
class DigDecompressError(GameDigError): ...
class DigSocketConnectError(GameDigError): ...
class SocketBindError(GameDigError): ...
class InvalidInputError(GameDigError): ...
class BadGameError(GameDigError): ...
class AutoQueryError(GameDigError): ...
class ProtocolFormatError(GameDigError): ...
class UnknownEnumCastError(GameDigError): ...
class JsonParseError(GameDigError): ...
class TypeParseError(GameDigError): ...
class HostLookupError(GameDigError): ...
