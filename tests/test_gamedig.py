import gamedig
import pytest


@pytest.mark.parametrize("invalid_game_id", ["", "not-a-game-id"])
def test_query_raises_for_invalid_game_ids(invalid_game_id):
    with pytest.raises(ValueError, match=f"Unknown game id: {invalid_game_id}"):
        gamedig.query(invalid_game_id, "127.0.0.1")


@pytest.mark.parametrize("invalid_address", ["", "not-an-ip-address"])
def test_query_raises_for_invalid_addresses(invalid_address):
    with pytest.raises(ValueError, match="invalid IP address syntax"):
        gamedig.query("minecraft", invalid_address)


def test_query_raises_for_auto_query_errors():
    with pytest.raises(gamedig.AutoQueryError):
        gamedig.query("minecraft", "127.0.0.1")


def test_query_raises_for_packet_errors():
    with pytest.raises(gamedig.PacketReceiveError):
        gamedig.query("csgo", "127.0.0.1")
