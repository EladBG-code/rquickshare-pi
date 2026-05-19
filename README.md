<div align="center">
  <img src="app/main/src-tauri/icons/rquickshare-pi.svg" width="140" alt="RQuickShare Pi logo" />

  <h1>RQuickShare Pi</h1>

  <p>
    <strong>Quick Share for Raspberry Pi OS ARM64, built and tested on real Pi hardware.</strong>
  </p>

  <p>
    <a href="https://github.com/EladBG-code/rquickshare-pi/releases">
      <img alt="Status: alpha" src="https://img.shields.io/badge/status-alpha-d12e5d?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://github.com/EladBG-code/rquickshare-pi/releases/latest">
      <img alt="Latest release" src="https://img.shields.io/github/v/release/EladBG-code/rquickshare-pi?include_prereleases&label=latest&style=flat-square&labelColor=3b363d&color=2f6df6">
    </a>
    <a href="https://github.com/EladBG-code/rquickshare-pi/actions/workflows/lint.yml">
      <img alt="Code quality" src="https://img.shields.io/github/actions/workflow/status/EladBG-code/rquickshare-pi/lint.yml?branch=master&label=code%20quality&style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://github.com/EladBG-code/rquickshare-pi/actions/workflows/build.yml">
      <img alt="Pi target policy" src="https://img.shields.io/github/actions/workflow/status/EladBG-code/rquickshare-pi/build.yml?branch=master&label=pi%20target&style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://github.com/EladBG-code/rquickshare-pi/blob/master/LICENSE">
      <img alt="License: GPL-3.0" src="https://img.shields.io/github/license/EladBG-code/rquickshare-pi?style=flat-square&labelColor=3b363d&color=1f9d55">
    </a>
    <a href="https://github.com/EladBG-code/rquickshare-pi/stargazers">
      <img alt="GitHub stars" src="https://img.shields.io/github/stars/EladBG-code/rquickshare-pi?style=flat-square&labelColor=3b363d&color=f6c343">
    </a>
    <a href="https://github.com/EladBG-code/rquickshare-pi/releases">
      <img alt="Release downloads" src="https://img.shields.io/github/downloads/EladBG-code/rquickshare-pi/total?style=flat-square&labelColor=3b363d&color=1f9d55">
    </a>
  </p>

  <p>
    <a href="https://www.raspberrypi.com/products/raspberry-pi-5/">
      <img alt="Raspberry Pi 5" src="https://img.shields.io/badge/Raspberry%20Pi-5-c51a4a?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://www.raspberrypi.com/software/operating-systems/">
      <img alt="Raspberry Pi OS 64-bit" src="https://img.shields.io/badge/Raspberry%20Pi%20OS-64--bit-c51a4a?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://en.wikipedia.org/wiki/AArch64">
      <img alt="Architecture: ARM64" src="https://img.shields.io/badge/arch-ARM64-2f6df6?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://www.debian.org/releases/bookworm/">
      <img alt="Debian Bookworm" src="https://img.shields.io/badge/Debian-Bookworm-a81d33?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://rust-lang.org/">
      <img alt="Rust" src="https://img.shields.io/badge/Rust-core-b7410e?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://v2.tauri.app/">
      <img alt="Tauri 2" src="https://img.shields.io/badge/Tauri-2-24c8db?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://vuejs.org/">
      <img alt="Vue 3" src="https://img.shields.io/badge/Vue-3-42b883?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://en.wikipedia.org/wiki/Bluetooth">
      <img alt="Bluetooth" src="https://img.shields.io/badge/Bluetooth-discovery-1672f3?style=flat-square&labelColor=3b363d">
    </a>
    <a href="https://en.wikipedia.org/wiki/Multicast_DNS">
      <img alt="mDNS" src="https://img.shields.io/badge/mDNS-networking-1f9d55?style=flat-square&labelColor=3b363d">
    </a>
  </p>

  <p>
    Hardware target: Raspberry Pi OS 64-bit on ARM64. Support is claimed only after local Pi build/run testing.
  </p>
</div>

## 🚀 What Is This?

RQuickShare Pi is a Raspberry Pi-focused fork of
[RQuickShare](https://github.com/Martichou/rquickshare), the open-source
Nearby Share / Quick Share desktop app.

This fork exists because Raspberry Pi support needs real ARM64 testing, native
Linux desktop dependencies, Bluetooth, mDNS, WebKitGTK, and patience. A GitHub
Actions build on x86_64 does not prove anything here. This repo is for the real
Pi target.

Current target:

- 🧠 Device: Raspberry Pi 5
- 🐧 OS: Raspberry Pi OS 64-bit / Debian Bookworm
- 🏗️ Architecture: `aarch64`
- 📦 App stack: Tauri 2 + Vue 3 + Rust core library
- 📡 Discovery stack: mDNS + Bluetooth advertisement

## ✅ Current Pi Status

The app has been built and started on a real Raspberry Pi 5.

Verified locally on the Pi:

- ✅ `core_lib` tests pass
- ✅ `core_lib` builds
- ✅ Tauri app checks
- ✅ Debian debug bundle builds
- ✅ App starts and reaches `RunEvent::Ready`
- ✅ TCP listener starts
- ✅ mDNS publishes a resolvable Quick Share service on the active LAN address
- ✅ BLE listener starts
- ✅ BLE visibility follows the app visibility state

Known rough edges:

- ⚠️ Full Tauri `targets = "all"` bundling can stall on non-Debian package
  formats. Use the Debian bundle while Pi support is being stabilized.
- ⚠️ Vue devtools Electron sidecar crashes on this Pi setup, so `pnpm dev`
  intentionally runs Tauri directly.
- ⚠️ Raspberry Pi support is experimental until repeated send/receive testing
  is complete.

## 🧬 Relationship To Upstream

This project is based on the excellent upstream work by Martin ANDRE:

https://github.com/Martichou/rquickshare

The goal is for this fork to become its own Pi-first project while keeping the
upstream license, credits, copyright notices, and project history intact.

Guardrails for this repo:

- 🛡️ `origin` is `EladBG-code/rquickshare-pi`
- 🔒 `upstream` is fetch-only locally; pushing to upstream is disabled
- 🔀 upstream changes should be reviewed and merged intentionally
- 🧭 this fork should not be blindly overwritten by the main RQuickShare project

## 🛠️ Build On Raspberry Pi OS

Install the latest public ARM64 release:

```bash
curl -fsSL https://eladbg-code.github.io/rquickshare-pi/install.sh | bash
```

This downloads the newest Raspberry Pi ARM64 `.deb` from GitHub Releases and
installs it with apt. A real APT repository is not published yet, so
`sudo apt install rquickshare-pi` is not available directly.

## 🧱 Build From Source

Fast path:

```bash
./install-rquickshare-pi.sh
```

The installer checks that it is running on ARM64, installs Pi dependencies,
enables Bluetooth and Avahi, installs Rust/Node/pnpm if needed, builds the
project, installs the `.deb`, and leaves the app available from the Accessories
menu.

Manual path:

Install system dependencies:

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
  avahi-utils \
  libavahi-client-dev
```

Enable runtime services:

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

Install Node.js 20 and pnpm 9.7.0:

```bash
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
sudo corepack enable
corepack prepare pnpm@9.7.0 --activate
```

Clone and build:

```bash
git clone git@github.com:EladBG-code/rquickshare-pi.git
cd rquickshare-pi
git switch pi-arm64-support
```

```bash
cd core_lib
cargo test
cargo build
```

```bash
cd ../app/main
pnpm install --frozen-lockfile
pnpm check
pnpm tauri build -d --bundles deb
```

The Debian bundle is written under:

```text
app/main/src-tauri/target/debug/bundle/deb/
```

## ▶️ Run On The Pi

For development:

```bash
cd app/main
WEBKIT_DISABLE_COMPOSITING_MODE=1 RUST_BACKTRACE=1 RUST_LOG=debug pnpm dev
```

For the built debug binary:

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 RUST_BACKTRACE=1 RUST_LOG=debug \
  ./app/main/src-tauri/target/debug/rquickshare-pi
```

If discovery is acting strange, check the Pi services:

```bash
bluetoothctl show
rfkill list bluetooth
systemctl status bluetooth --no-pager
systemctl status avahi-daemon --no-pager
ip addr
```

## 📁 Important Paths

Settings on Linux:

```text
~/.local/share/dev.eladbg.rquickshare-pi/.settings.json
```

Main app:

```text
app/main
```

Rust core:

```text
core_lib
```

Canonical logo source:

```text
app/main/src-tauri/icons/rquickshare-pi.svg
```

## 🧪 Test Notes

Real support means testing on the Pi, not just compiling somewhere else.

Useful commands:

```bash
uname -m
cat /etc/os-release
rustc -Vv
cargo -V
node -v
pnpm -v
protoc --version
```

Expected architecture:

```text
aarch64
```

More detailed build notes live in:

```text
PI_BUILD_NOTES.md
```

## 🧯 Troubleshooting

### Samsung phone cannot see the Pi

If a Samsung phone opens Quick Share and immediately drops/disconnects Wi-Fi,
turn off Samsung's Apple-device compatibility mode:

```text
Settings > Connected devices > Quick Share > Share with Apple devices > Off
```

Why this matters:

- Android Quick Share discovers nearby devices with Bluetooth, then transfers
  over a direct local Wi-Fi connection.
- This fork currently supports the Wi-Fi LAN / mDNS Quick Share path.
- Samsung's Apple compatibility mode can push the phone into a different
  sharing path, which prevents the phone from seeing this Pi receiver.

After disabling that option, keep Bluetooth and Wi-Fi enabled, keep the phone
and Pi nearby, then reopen the Android share sheet and choose Quick Share.

References:

- [Android Quick Share FAQ](https://www.android.com/quick-share/with-android/)
- [Samsung Quick Share Wi-Fi note](https://www.samsung.com/levant/support/mobile-devices/quick-share-cannot-be-used-when-wi-fi-is-off-and-wi-fi-turns-on-when-setting-who-can-share-with-you-option/)
- [Packet Linux Quick Share requirements](https://github.com/nozwock/packet#requirements)

### Raspberry Pi/WebKit window corruption

The installed Debian release applies the Raspberry Pi WebKitGTK compositor
workaround automatically before the Tauri webview starts. This fixes the Pi
window corruption issue where the first visible frame could appear as
horizontal stripes until manually resized.

If you are running from source or debugging manually, keep the environment
variable in your command:

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 pnpm dev
```

Need a static firewall port:

```bash
vim ~/.local/share/dev.eladbg.rquickshare-pi/.settings.json
```

Example:

```json
{
  "port": 12345
}
```

App stays alive after closing the window:

```bash
ps aux | grep rquickshare-pi
```

That can be normal if the tray process is still running.

## 📜 License

This project keeps the upstream license:

```text
GPL-3.0
```

RQuickShare Pi includes modifications by EladBG-code. Original RQuickShare
copyrights, author notices, license terms, and project history remain intact.

Additional legal and trademark notices are in [LEGAL_NOTICE.md](LEGAL_NOTICE.md).

## 🙏 Credits

RQuickShare Pi would not exist without the upstream project and the wider open
source Nearby Share ecosystem:

- https://github.com/Martichou/rquickshare
- https://github.com/grishka/NearDrop
- https://github.com/vicr123/QNearbyShare

## 🌱 Direction

This fork starts as Raspberry Pi compatibility work, but it is intended to grow
into a Pi-first app with its own identity, release flow, and hardware-tested
support story.

## Star History

<a href="https://www.star-history.com/?repos=EladBG-code%2Frquickshare-pi&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=EladBG-code/rquickshare-pi&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=EladBG-code/rquickshare-pi&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=EladBG-code/rquickshare-pi&type=date&legend=top-left" />
 </picture>
</a>

## ❤️ If you feel like supporting me for more stuff like this

<a href="https://ko-fi.com/eladbg">
  <img width="600" height="300" alt="eladbg-Sharable-Profile)-Horizontal copy" src="https://github.com/user-attachments/assets/6d59980e-deeb-461f-a1a0-df62a0a030cb" href="https://ko-fi.com/eladbg"/>
</a>
