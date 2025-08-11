# pyfalcon

A cross-version Python disassembler that supports both command-line and web interfaces.

### Web Version

Visit the [live demo](https://pyfalcon.svenskithesource.be) and drag & drop your `.pyc` files directly into the browser.

### Install
```bash
# Install from crates.io
cargo install pyfalcon

# Checkout the help command to see what options are available
pyfalcon --help

# Disassemble input.pyc
pyfalcon input.pyc
```

### Local usage
You also have the option to clone the repository and run it directly.

```bash
# Run the CLI tool after cloning
cargo run --bin pyfalcon -- input.pyc
```

## Building

To build the web version you will need to install trunk (`cargo install trunk`).

```bash
# CLI tool
cargo build --release --bin pyfalcon

# Web version
cd wasm && trunk build --release
```

## Supported Python Versions

Currently supports Python 3.10, with more versions planned for the future.