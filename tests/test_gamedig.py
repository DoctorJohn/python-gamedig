import socket
import subprocess
import sys

import pytest

import gamedig


@pytest.mark.parametrize("invalid_game_id", ["", "not-a-game-id"])
def test_query_raises_for_invalid_game_ids(invalid_game_id):
    with pytest.raises(ValueError, match=f"Unknown game id: {invalid_game_id}"):
        gamedig.query(invalid_game_id, "127.0.0.1")


@pytest.mark.parametrize("invalid_address", ["", "not-an-ip-address"])
def test_query_raises_for_invalid_addresses(invalid_address):
    with pytest.raises(ValueError, match="invalid IP address syntax"):
        gamedig.query("minecraft", invalid_address)


def test_query_connect_timeout_duration_must_not_be_0():
    with pytest.raises(
        gamedig.InvalidInputError, match="Connect duration must not be 0"
    ):
        gamedig.query(
            "minecraft",
            "127.0.0.1",
            timeout_settings={"retries": 0, "connect": 0},
        )


def test_query_read_timeout_duration_must_not_be_0():
    with pytest.raises(gamedig.InvalidInputError, match="Read duration must not be 0"):
        gamedig.query(
            "minecraft",
            "127.0.0.1",
            timeout_settings={"retries": 0, "read": 0},
        )


def test_query_write_timeout_duration_must_not_be_0():
    with pytest.raises(gamedig.InvalidInputError, match="Write duration must not be 0"):
        gamedig.query(
            "minecraft",
            "127.0.0.1",
            timeout_settings={"retries": 0, "write": 0},
        )


def test_query_timeout_settings_retries_are_optional():
    with pytest.raises(gamedig.PacketReceiveError):
        gamedig.query("csgo", "127.0.0.1", timeout_settings={"read": 1})


def test_query_uses_default_timeouts_for_omitted_timeout_settings():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as silent_server:
        silent_server.bind(("127.0.0.1", 0))
        port = silent_server.getsockname()[1]
        code = f"import gamedig; gamedig.query('csgo', '127.0.0.1', {port}, {{'retries': 0}})"

        # A hanging query can't be interrupted from within the same process
        result = subprocess.run(
            [sys.executable, "-c", code],
            capture_output=True,
            check=False,
            text=True,
            timeout=15,
        )

    assert "gamedig.PacketReceiveError" in result.stderr


def test_query_raises_for_auto_query_errors():
    with pytest.raises(gamedig.AutoQueryError):
        gamedig.query("minecraft", "127.0.0.1")


def test_query_raises_for_packet_errors():
    with pytest.raises(gamedig.PacketReceiveError):
        gamedig.query("csgo", "127.0.0.1")
