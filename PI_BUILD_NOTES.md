# Raspberry Pi OS ARM64 Build Notes

These notes are for an experimental Raspberry Pi compatibility branch based on
the official RQuickShare project:

https://github.com/Martichou/rquickshare

This is not an official RQuickShare release. Keep the upstream license,
copyright notices, authors, credits, and project history intact.

## Target

- Device: Raspberry Pi 5
- OS: Raspberry Pi OS 64-bit
- Architecture: ARM64 / aarch64
- Development host: Windows 11 x86_64 / amd64

The Windows machine is useful for editing, GitHub setup, and documentation, but
it cannot prove that RQuickShare builds or runs on Raspberry Pi OS ARM64. Real
validation must happen on the Raspberry Pi.

## What the Project Uses

The current upstream project structure is:

- `core_lib`: Rust library containing the QuickShare discovery, connection, and
  transfer logic.
- `app/main`: Tauri 2 desktop application using Vue 3, Vite, TypeScript, and
  the Rust library.
- `app/main/package.json`: pnpm project, pinned to `pnpm@9.7.0`.
- `core_lib/package.json`: npm metadata for the TypeScript bindings/dist files.
- `app/main/src-tauri/rust-toolchain`: `nightly`.
- `.github/workflows`: current Linux jobs run on GitHub-hosted Ubuntu runners,
  which are amd64 by default, not Raspberry Pi ARM64.

Native Linux pieces seen in the project and existing workflows include:

- GTK 3
- WebKitGTK 4.1
- libsoup 3
- librsvg
- Ayatana AppIndicator
- D-Bus
- Protocol Buffers compiler
- Bluetooth through `bluer` and `btleplug`
- mDNS through `mdns-sd`

`core_lib/Cargo.toml` already has an aarch64 Linux-specific `dbus` dependency
with the `vendored` feature. That is useful, but it is not the same as proving
the full Tauri app works on Raspberry Pi OS.

## Prepare the Raspberry Pi

Run these commands on the Raspberry Pi 5, not on Windows.

```bash
sudo apt update
sudo apt install -y \
  git \
  curl \
  build-essential \
  pkg-config \
  libssl-dev \
  protobuf-compiler \
  libprotobuf-dev \
  libdbus-1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev \
  libwebkit2gtk-4.1-dev \
  bluez \
  avahi-daemon \
  libavahi-client-dev
```

Enable the runtime services that are likely to matter while testing discovery
and Bluetooth advertisement:

```bash
sudo systemctl enable --now bluetooth
sudo systemctl enable --now avahi-daemon
```

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup toolchain install stable
rustup toolchain install nightly
rustup default stable
```

Install Node.js 20 and pnpm:

```bash
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
sudo corepack enable
corepack prepare pnpm@9.7.0 --activate
```

Check the environment:

```bash
uname -m
cat /etc/os-release
rustc -Vv
cargo -V
node -v
pnpm -v
protoc --version
```

`uname -m` should report `aarch64`.

## Clone This Fork on the Pi

If you want to push changes from the Pi, authenticate GitHub first. For
read-only testing after the repository is public, authentication is not required.
One authenticated option is GitHub CLI:

```bash
gh auth login
gh repo clone EladBG-code/rquickshare-pi
cd rquickshare-pi
git switch pi-arm64-support
git remote add upstream https://github.com/Martichou/rquickshare.git
```

If you use SSH instead, clone with your GitHub SSH key:

```bash
git clone git@github.com:EladBG-code/rquickshare-pi.git
cd rquickshare-pi
git switch pi-arm64-support
git remote add upstream https://github.com/Martichou/rquickshare.git
```

Verify the remotes:

```bash
git remote -v
```

Expected remotes:

- `origin`: your RQuickShare Pi fork.
- `upstream`: the official RQuickShare repository.

## Build on the Pi

Start with the Rust library:

```bash
cd core_lib
cargo test
cargo build
cd ..
```

Then build the Tauri app:

```bash
cd app/main
pnpm install --frozen-lockfile
pnpm check
pnpm tauri build -d --bundles deb
```

The full Tauri `targets = "all"` build currently attempts more package formats
than the Pi needs. Prefer the Debian bundle while Raspberry Pi support is being
validated.

For a full release package attempt:

```bash
pnpm build
```

For a development run with useful logs:

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 RUST_BACKTRACE=1 RUST_LOG=debug pnpm dev
```

## Collect Build or Runtime Errors

If the build fails, collect the full command output:

```bash
cd app/main
RUST_BACKTRACE=1 RUST_LOG=debug pnpm tauri build -d --bundles deb 2>&1 | tee "$HOME/rquickshare-pi-build.log"
```

For runtime discovery, Bluetooth, or mDNS issues, collect:

```bash
uname -a
cat /etc/os-release
bluetoothctl show
rfkill list bluetooth
systemctl status bluetooth --no-pager
systemctl status dbus --no-pager
systemctl status avahi-daemon --no-pager
ip addr
```

## Troubleshooting

### WebKitGTK 4.1 package not found

Raspberry Pi OS based on Debian Bookworm should provide the WebKitGTK 4.1
development packages used by the current upstream CI. Check availability with:

```bash
apt-cache policy libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev
```

If those packages are missing, confirm the Raspberry Pi OS release with
`cat /etc/os-release` before changing the Tauri/WebKit dependency plan.

### `protoc` not found

Install Protocol Buffers tooling:

```bash
sudo apt install -y protobuf-compiler libprotobuf-dev
```

### No Bluetooth adapter or Bluetooth permission errors

Check that the Pi sees the controller and that it is not blocked:

```bash
bluetoothctl list
bluetoothctl show
rfkill list bluetooth
sudo systemctl status bluetooth --no-pager
```

If Bluetooth is soft-blocked:

```bash
sudo rfkill unblock bluetooth
sudo systemctl restart bluetooth
```

### D-Bus errors

The Linux Bluetooth stack is exposed through D-Bus. Confirm the system service
is running:

```bash
systemctl status dbus --no-pager
systemctl status bluetooth --no-pager
```

### Devices do not appear

RQuickShare currently depends on local network discovery. Check that both
devices are on the same network and that multicast DNS traffic is not blocked.
On the Pi, also check:

```bash
systemctl status avahi-daemon --no-pager
hostname -I
```

### GitHub Actions are not proof of Pi support

The current GitHub-hosted Linux runners are amd64 unless explicitly configured
otherwise. A successful GitHub Actions Linux build does not prove that the app
builds or runs on Raspberry Pi OS ARM64.

## Current Raspberry Pi TODOs

- Build `core_lib` on a Raspberry Pi 5 running Raspberry Pi OS 64-bit.
- Build `app/main` on the Pi with pnpm and Tauri.
- Confirm whether `.deb`, `.AppImage`, or other Tauri bundles are produced for
  ARM64 and named clearly.
- Run the app on the Pi desktop session.
- Test Bluetooth advertisement, D-Bus access, and mDNS discovery with an Android
  Quick Share device.
- Capture and fix any Pi-specific runtime errors without claiming support before
  the Pi test passes.
