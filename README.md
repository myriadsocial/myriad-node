<div align="center">
<img src="https://avatars.githubusercontent.com/u/80524516?s=200&v=4">
</div>
<br>
<br>

<div align="Center">
<h1>Myriad Node</h1>
<h2>It's Your Turn to Own Your Web</h2>
Starting with Myriad.Social, we are creating a platform where social app, metaverse and messenger seamlessly integrate, together and with further applications. As a user, a content creator or a builder, Myriad is designed to be yours.
<br>
<br>

[![Substrate version](https://img.shields.io/badge/Substrate-3.0.0-brightgreen?logo=Parity%20Substrate)](https://substrate.dev/)
[![Medium](https://img.shields.io/badge/Medium-Myriad-brightgreen?logo=medium)](https://medium.com/@myriadsocial.blog)
</div>

---

## Getting Started

Follow these steps to get started with the Node

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/debug/myriad \
--base-path .local \
--dev \
--alice
```

Purge the development chain's state:

```bash
./target/debug/myriad \
purge-chain \
--base-path .local \
--dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/debug/myriad \
--base-path .local \
--dev \
--alice \
-lruntime=debug
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./.maintain/docker/start-docker-compose.sh
```
