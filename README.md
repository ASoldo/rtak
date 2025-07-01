# RTAK - Rust TAK Server

RTAK is a full-featured TAK Server implemented in Rust, designed to receive Cursor on Target (CoT) messages over UDP and broadcast them in real-time to connected WebSocket clients. It’s inspired by ATAK/TAK Server but implemented with modern Rust technologies like Actix, Tokio, and Serde.

---

## Features

* Listens for CoT messages on UDP
* Parses CoT XML to Rust structs
* Broadcasts CoT messages to all connected WebSocket clients
* REST API health check endpoint
* Built with Actix-Web, Actix actors, and Tokio async runtime

---

## Project structure

```
src/
  api.rs           # WebSocket and REST server
  broadcaster.rs   # Actix actor to manage connected WebSocket sessions
  config.rs        # Loads configuration from rtak.toml
  cot.rs           # CoT data structures
  udp.rs           # UDP CoT listener
  main.rs          # Application entry point
rtak.toml          # Configuration file
Cargo.toml         # Rust package manifest
```

---

## Requirements

* Rust (stable toolchain recommended)
* [websocat](https://github.com/vi/websocat) for WebSocket testing
* netcat (`nc`) for sending CoT messages over UDP

---

## Building

```bash
cargo build --release
```

---

## Running

```bash
cargo run
```

You should see logs like:

```
INFO rtak: Configuration loaded: Config { udp_bind: "0.0.0.0:6969", rest_bind: "0.0.0.0:8080" }
INFO actix_server::server: starting service: "actix-web-service-0.0.0.0:8080" ...
INFO rtak::udp: UDP listener bound to 0.0.0.0:6969
```

---

## Health check

Verify your server is up with:

```bash
curl http://127.0.0.1:8080/health
```

Expected response:

```
RTAK is alive!
```

---

## Connecting a WebSocket client

Use websocat to connect to the server’s WebSocket endpoint:

```bash
websocat ws://127.0.0.1:8080/ws
```

---

## Sending CoT messages for testing

In a separate terminal, send a test CoT message via UDP using netcat:

```bash
echo '<event version="2.0" uid="test-123" type="a-f-G-U-C" how="m-g" time="2025-07-01T14:30:00Z" start="2025-07-01T14:30:00Z" stale="2025-07-01T14:35:00Z"><point lat="45.0" lon="16.0" hae="0.0" ce="9999999" le="9999999"/></event>' | nc -u -w1 127.0.0.1 6969
```

Your connected WebSocket client should receive the exact CoT XML you sent.

---

## Configuration

Edit `rtak.toml` to set the UDP bind address and REST server bind address. Example:

```toml
udp_bind = "0.0.0.0:6969"
rest_bind = "0.0.0.0:8080"
```

---

## Status

* Core CoT receive and broadcast working
* Planned: CoT database storage, federation, authentication, plugin system

---

## License

MIT

