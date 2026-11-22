"""A Valve server without players and rules that reports app id 0.

Prints the UDP port it listens on, then answers queries until it is killed.
"""

import socket

HEADER = b"\xff\xff\xff\xff"
INFO = (
    b"I"
    b"\x11"  # protocol version
    b"name\x00map\x00folder\x00game\x00"
    b"\x00\x00"  # app id
    b"\x00\x08\x00"  # players, max players, bots
    b"dl"  # dedicated server, linux
    b"\x00\x00"  # no password, no VAC
    b"1.0\x00"  # game version
)
RESPONSES = {
    ord("T"): INFO,
    ord("U"): b"D\x00",  # no players
    ord("V"): b"E\x00\x00",  # no rules
}


def main():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        sock.bind(("127.0.0.1", 0))
        print(sock.getsockname()[1], flush=True)
        while True:
            request, address = sock.recvfrom(1400)
            sock.sendto(HEADER + RESPONSES[request[4]], address)


if __name__ == "__main__":
    main()
