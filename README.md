# budget-rs

A budget management web application built with Rust and WebAssembly.

## Technology Stack

- **Yew** - Rust framework for building web applications
- **Trunk** - Build tool for Rust WASM applications
- **WebAssembly** - Compilation target for browser execution
- **Node.js** - JavaScript runtime for development tools
- **workbox-cli** - Service worker generation for PWA support

## Commands

### Development

Before the first run, generate local HTTPS certificates (the `certs/` folder is gitignored and must not be committed).

1. Install [mkcert](https://github.com/FiloSottile/mkcert):
   - macOS: `brew install mkcert`
   - Windows: `choco install mkcert` or `winget install FiloSottile.mkcert`
2. Install the local CA into the system trust store:
   ```bash
   mkcert -install
   ```
3. Generate certificates for localhost:
   ```bash
   mkcert -cert-file certs/cert.pem -key-file certs/key.pem localhost 127.0.0.1
   ```

Start the development server (HTTPS via Trunk):
```bash
npm run dev
# or
trunk serve
```

It serves on `https://localhost:8000/`. Paths to the certificates are configured in `Trunk.toml` (`tls_cert_path` / `tls_key_path`).

### Building

Build the project for production:
```bash
trunk build --release
```

Build the project (debug mode):
```bash
trunk build
```

### Rust/Cargo Commands

Check the project for errors:
```bash
cargo check
```

Build the Rust project:
```bash
cargo build
```

Build for WebAssembly target:
```bash
cargo build --target wasm32-unknown-unknown
```

Run tests:
```bash
cargo test
```

Format code:
```bash
cargo fmt
```

Lint code:
```bash
cargo clippy
```

## Installation

### Install Rust wasm-bindgen manually

Download the latest wasm-bindgen-cli Windows binary from the [GitHub releases page](https://github.com/wasm-bindgen/wasm-bindgen/releases). Select the .exe file matching your architecture (e.g., wasm-bindgen-cli-x86_64-pc-windows-msvc.zip for 64-bit).

Unzip the downloaded archive to the cargo bin directory `C:\Users\xxx\.cargo\bin`

Open a new Command Prompt and run `wasm-bindgen --version` to confirm it works.
