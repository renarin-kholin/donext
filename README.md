# DoNext

DoNext is a simple task tracker that helps you focus on what to do next. 
This project uses [vizia](https://github.com/vizia/vizia) (Declarative GUI library written in Rust) for its GUI.

## Requirements

- Rust (latest stable)
- Cargo

## Build Instructions

```bash
cargo build --release
```

## Run

```bash
cargo run --release
```

Data is persisted automatically to `~/.local/share/donext/data.json`.

## License
This code is licensed under the GNU GPLv3 license. 
