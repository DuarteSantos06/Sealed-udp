# sealed_udp

A small file-transfer protocol over UDP, written in Rust from scratch as a learning project. Each connection starts with an X25519 Diffie-Hellman handshake, so client and server derive the same shared secret without sending it over the network. The plan is to use that secret to encrypt everything that follows.

> **Status: work in progress.** The handshake and the plaintext request/response flow work. Encryption is **not** implemented yet, so data packets travel in the clear. Do not use this for anything real.

## What works today

- **Handshake:** the client sends a `ClientHello` and the server replies with a `ServerHello`. Both sides compute the same 32-byte shared secret.
- **Data packets:** an outer envelope that carries a connection ID and an application message.
- **Messages:** the client sends a `Request` for a file chunk by name, offset and length. The server reads the chunk and replies with a `Response`.
- **Path traversal guard:** the server rejects file names that contain `..` or `/`.

## Project layout

```
src/
├── lib.rs          # exports the modules below
├── protocol.rs     # outer packets: ClientHello, ServerHello, Data (parse + serialize)
├── message.rs      # inner messages carried in Data packets: Request, Response
├── crypto.rs       # X25519 keypairs, shared secret, random nonces and IDs
└── bin/
    ├── server.rs   # UDP server that answers file chunk requests
    └── client.rs   # UDP client that performs the handshake and downloads a chunk
```

## Getting started

You need a Rust toolchain that supports **edition 2024** (Rust 1.85 or newer).

The server only serves files from a `files/` folder at the project root, and that folder is not in the repository. Create it and add the file the client asks for (`test.txt` by default):

```sh
mkdir files
echo "hello from sealed_udp" > files/test.txt
```

Then start the server and the client from the project root:

```sh
# Terminal 1: start the server (listens on 127.0.0.1:8080)
cargo run --bin server

# Terminal 2: run the client
cargo run --bin client
```

The client performs the handshake, requests `test.txt` from the server and saves the bytes it receives to `downloaded.txt` in the project root. For now it downloads a single chunk of up to 1024 bytes, so only the first 1024 bytes of larger files are saved.

To download a different file, put it in `files/` and change the file name passed to `send_request` in `src/bin/client.rs`. File names must be plain names: the server rejects any name that contains `/` or `..`.

## Wire format

All multi-byte integers are big-endian.

### Packets (`protocol.rs`)

| Type          | Byte   |
|---------------|--------|
| `ClientHello` | `0x01` |
| `ServerHello` | `0x02` |
| `Data`        | `0x03` |

**Handshake (`ClientHello` / `ServerHello`): exactly 74 bytes**

| Offset | Size | Field           |
|-------:|-----:|-----------------|
| 0      | 1    | type            |
| 1      | 1    | version (`0x01`)|
| 2      | 8    | connection ID   |
| 10     | 32   | nonce           |
| 42     | 32   | X25519 public key |

The client picks a random connection ID, and the server echoes it back in the `ServerHello`.

**Data: 9-byte header plus a payload of up to 1200 bytes**

| Offset | Size     | Field         |
|-------:|---------:|---------------|
| 0      | 1        | type (`0x03`) |
| 1      | 8        | connection ID |
| 9      | ≤ 1200   | payload (a message) |

### Messages (`message.rs`)

These are carried inside the payload of a `Data` packet.

**Request (`0x04`)**

| Offset | Size     | Field                |
|-------:|---------:|----------------------|
| 0      | 1        | type (`0x04`)        |
| 1      | 8        | request ID           |
| 9      | 8        | offset (`u64`)       |
| 17     | 4        | length (`u32`)       |
| 21     | variable | file name (UTF-8)    |

**Response (`0x05`)**

| Offset | Size     | Field                |
|-------:|---------:|----------------------|
| 0      | 1        | type (`0x05`)        |
| 1      | 8        | request ID (copied from the request) |
| 9      | 8        | offset (`u64`)       |
| 17     | ≤ 1024   | file data            |

## Dependencies

- [`x25519-dalek`](https://crates.io/crates/x25519-dalek) for X25519 key exchange
- [`rand_core`](https://crates.io/crates/rand_core) with `getrandom` for OS randomness
