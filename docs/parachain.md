# Myriad Node Parachain

## Development

### Build

```bash
cargo build -p myriad-parachain
```

### Run

```bash
./target/debug/myriad-parachain \
--base-path .local/parachain \
--dev \
--alice \
--force-authoring \
--parachain-id 2000 \
-- \
--execution wasm \
--chain rococo
```
