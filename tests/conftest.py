import subprocess
import sys
from pathlib import Path

import pytest


@pytest.fixture
def valve_server_port():
    # Runs in its own process, as queries hold the GIL
    server_path = Path(__file__).with_name("valve_server.py")
    with subprocess.Popen(
        [sys.executable, server_path], stdout=subprocess.PIPE
    ) as server:
        assert server.stdout
        yield int(server.stdout.readline())
        server.kill()
