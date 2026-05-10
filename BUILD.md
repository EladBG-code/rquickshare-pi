The project is divided into two parts:

- **core_lib:** This is a Rust library that encompasses all the logic necessary for discovering, connecting to, and transferring files to QuickShare-compatible clients.
- **app/main:** A Tauri application that utilizes core_lib to handle incoming requests and initiate outgoing ones.

How to build on Raspberry Pi OS ARM64
-------------------------------------

The friendly path is:

```bash
./install-rquickshare-pi.sh
```

That script installs the required Raspberry Pi OS packages, enables Bluetooth
and Avahi, installs Rust/Node/pnpm if needed, builds the project, and installs
the generated Debian package.

### core_lib

Building the core_lib is straightforward because it is a basic Rust project.

Install `protobuf-compiler` system package, and then simply run `cargo build` or `cargo build --release` from `core_lib` folder.

### app/main

The app/main is developed as a Tauri application. For package management, pnpm is recommended (though npm and others may also work, pnpm is preferred for this project).

(all commands are run inside the `app/main` folder)

First, install the necessary dependencies:

```
pnpm install --frozen-lockfile
```

- To run the debug version:

```
pnpm dev
```

- To build a Raspberry Pi Debian package:

```
pnpm tauri build --bundles deb
```

For more detailed information on building the app/main and understanding any potential limitations, it’s advised to consult the [Tauri documentation](https://v2.tauri.app/start).

This fork only publishes Raspberry Pi OS ARM64 support claims after local Pi
build/run testing.
