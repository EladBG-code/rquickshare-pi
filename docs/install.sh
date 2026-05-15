#!/usr/bin/env bash
set -Eeuo pipefail

REPO="EladBG-code/rquickshare-pi"
API_URL="https://api.github.com/repos/${REPO}/releases"
DRY_RUN=0

step() {
	printf '\n==> %s\n' "$1"
}

fail() {
	printf 'RQuickShare Pi installer: %s\n' "$1" >&2
	exit 1
}

need_command() {
	command -v "$1" >/dev/null 2>&1
}

case "${1:-}" in
	"") ;;
	--dry-run) DRY_RUN=1 ;;
	-h|--help)
		printf 'Usage: install.sh [--dry-run]\n'
		exit 0
		;;
	*) fail "unknown option: ${1}" ;;
esac

arch="$(dpkg --print-architecture 2>/dev/null || uname -m)"
case "$arch" in
	arm64|aarch64) ;;
	*) fail "this package targets Raspberry Pi OS 64-bit on ARM64. Detected: ${arch}" ;;
esac

need_command curl || fail "curl is required. Install it with: sudo apt install curl"
need_command python3 || fail "python3 is required. Install it with: sudo apt install python3"
need_command sudo || fail "sudo is required for apt installation"

step "Finding the latest RQuickShare Pi ARM64 release"
release_json="$(curl -fsSL "${API_URL}")"
asset_url="$(
	printf '%s' "${release_json}" | python3 -c '
import json
import re
import sys

releases = json.load(sys.stdin)
asset_pattern = re.compile(r"^RQuickShare\.Pi_.*_arm64\.deb$")

for release in releases:
    if release.get("draft"):
        continue
    for asset in release.get("assets", []):
        name = asset.get("name", "")
        if asset_pattern.match(name):
            print(asset["browser_download_url"])
            raise SystemExit(0)

raise SystemExit(1)
'
)" || fail "could not find an ARM64 .deb release asset"

if [[ "${DRY_RUN}" == "1" ]]; then
	printf 'Latest package: %s\n' "${asset_url}"
	exit 0
fi

tmp_dir="$(mktemp -d)"
trap 'rm -rf "${tmp_dir}"' EXIT
deb_path="${tmp_dir}/RQuickShare.Pi_latest_arm64.deb"

step "Downloading latest package"
curl -fL "${asset_url}" -o "${deb_path}"

step "Installing with apt"
sudo apt install -y "${deb_path}"

step "Done"
printf 'RQuickShare Pi is installed. Open it from Accessories or the system tray.\n'
