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

Start the development server:
```bash
npm run dev
```
This runs both the Trunk dev server and SSL proxy concurrently.

Start only the Trunk development server:
```bash
npm run trunk-dev
# or
trunk serve
```

Start only the SSL proxy (for HTTPS on port 8443):
```bash
npm run ssl-proxy
```

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
