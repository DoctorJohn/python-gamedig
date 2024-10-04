# python-gamedig

Unofficial high-level Python bindings for the Rust [gamedig](https://crates.io/crates/gamedig) crate.

## Installation

```bash
pip install gamedig
```

## Usage

```python
from socket import gethostbyname
from gamedig import query

ip_address = gethostbyname('minecraftonline.com')

response = query('minecraft', ip_address)

print(response)
```
