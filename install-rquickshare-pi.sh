#!/usr/bin/env bash
set -Eeuo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$REPO_DIR/app/main"
CORE_DIR="$REPO_DIR/core_lib"
PNPM_VERSION="9.7.0"

step() {
  printf '\n==> %s\n' "$1"
}

need_command() {
  command -v "$1" >/dev/null 2>&1
}

is_node_20() {
  need_command node && node -v | grep -Eq '^v20\.'
}

step "Checking Raspberry Pi target"
arch="$(uname -m)"
if [[ "$arch" != "aarch64" && "$arch" != "arm64" ]]; then
  echo "This installer is intended for Raspberry Pi OS 64-bit on ARM64."
  echo "Detected architecture: $arch"
  exit 1
fi

if [[ -r /etc/os-release ]]; then
  . /etc/os-release
  echo "Detected OS: ${PRETTY_NAME:-unknown}"
fi

step "Preparing sudo"
sudo -v

step "Installing Raspberry Pi OS dependencies"
sudo apt-get update
sudo apt-get install -y \
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

step "Enabling runtime services"
sudo systemctl enable --now bluetooth
sudo systemctl enable --now avahi-daemon
sudo rfkill unblock bluetooth || true

step "Installing Rust if needed"
if ! need_command rustc || ! need_command cargo; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

# shellcheck disable=SC1091
. "$HOME/.cargo/env"
rustup toolchain install stable
rustup toolchain install nightly
rustup default stable

step "Installing Node.js 20 and pnpm $PNPM_VERSION"
if ! is_node_20; then
  curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
  sudo apt-get install -y nodejs
fi
sudo corepack enable
corepack prepare "pnpm@$PNPM_VERSION" --activate

step "Building and testing core library"
cd "$CORE_DIR"
cargo test
cargo build

step "Building RQuickShare Pi Debian package"
cd "$APP_DIR"
pnpm install --frozen-lockfile
pnpm check
WEBKIT_DISABLE_COMPOSITING_MODE=1 RUST_BACKTRACE=1 RUST_LOG=debug pnpm tauri build -d --bundles deb

step "Installing the Debian package"
deb_file="$(find "$APP_DIR/src-tauri/target/debug/bundle/deb" -maxdepth 1 -name '*.deb' -printf '%T@ %p\n' | sort -nr | awk 'NR==1 {print $2}')"
if [[ -z "$deb_file" ]]; then
  echo "No .deb file was produced."
  exit 1
fi

sudo dpkg -i "$deb_file"
sudo apt-get install -f -y

if need_command update-desktop-database; then
  sudo update-desktop-database /usr/share/applications || true
fi

step "Installed"
echo "Launch RQuickShare Pi from the Accessories menu, or run:"
echo "  WEBKIT_DISABLE_COMPOSITING_MODE=1 rquickshare-pi"
