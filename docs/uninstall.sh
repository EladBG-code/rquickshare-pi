#!/usr/bin/env bash
set -Eeuo pipefail

PACKAGE_NAME="r-quick-share-pi"
APP_BIN="rquickshare-pi"
APP_CONFIG_DIR="${HOME}/.config/dev.eladbg.rquickshare-pi"
APP_CACHE_DIR="${HOME}/.cache/dev.eladbg.rquickshare-pi"
APP_LOG_DIR="${HOME}/.local/state/dev.eladbg.rquickshare-pi"
AUTOSTART_DIR="${HOME}/.config/autostart"
PURGE_USER_DATA=0
DRY_RUN=0

step() {
	printf '\n==> %s\n' "$1"
}

info() {
	printf '%s\n' "$1"
}

fail() {
	printf 'RQuickShare Pi uninstaller: %s\n' "$1" >&2
	exit 1
}

need_command() {
	command -v "$1" >/dev/null 2>&1
}

run() {
	if [[ "${DRY_RUN}" == "1" ]]; then
		printf '[dry-run] %q' "$1"
		shift
		printf ' %q' "$@"
		printf '\n'
		return 0
	fi

	"$@"
}

usage() {
	cat <<'EOF'
Usage: uninstall.sh [--purge-user-data] [--dry-run]

Removes the RQuickShare Pi Debian package and stale autostart entries.

Options:
  --purge-user-data  Also remove RQuickShare Pi settings, cache, and reports.
  --dry-run          Print the actions without changing the system.
  -h, --help         Show this help text.
EOF
}

while [[ "$#" -gt 0 ]]; do
	case "$1" in
		--purge-user-data) PURGE_USER_DATA=1 ;;
		--dry-run) DRY_RUN=1 ;;
		-h|--help)
			usage
			exit 0
			;;
		*) fail "unknown option: $1" ;;
	esac
	shift
done

need_command sudo || fail "sudo is required for apt removal"
need_command apt || fail "apt is required for package removal"
need_command dpkg || fail "dpkg is required to check package state"

if need_command systemctl; then
	step "Stopping user service if it exists"
	run systemctl --user stop rquickshare-pi-dev.service 2>/dev/null || true
fi

if need_command pgrep && need_command pkill && pgrep -x "${APP_BIN}" >/dev/null 2>&1; then
	step "Stopping running app process"
	run pkill -x "${APP_BIN}" || true
fi

if dpkg -s "${PACKAGE_NAME}" >/dev/null 2>&1; then
	step "Removing ${PACKAGE_NAME}"
	run sudo apt remove -y "${PACKAGE_NAME}"
else
	step "Package not installed"
	info "${PACKAGE_NAME} is not installed through apt."
fi

if [[ -d "${AUTOSTART_DIR}" ]]; then
	step "Removing stale autostart launchers"
	while IFS= read -r -d '' desktop_file; do
		if grep -qiE "rquickshare-pi|RQuickShare Pi" "${desktop_file}"; then
			run rm -f "${desktop_file}"
		fi
	done < <(find "${AUTOSTART_DIR}" -maxdepth 1 -type f -name '*.desktop' -print0)
fi

if [[ "${PURGE_USER_DATA}" == "1" ]]; then
	step "Removing app settings, cache, and reports"
	run rm -rf "${APP_CONFIG_DIR}" "${APP_CACHE_DIR}" "${APP_LOG_DIR}"
else
	step "Keeping user data"
	info "Settings, cache, reports, and received files were left alone."
	info "Run again with --purge-user-data only if you intentionally want app state removed."
fi

step "Done"
info "RQuickShare Pi has been removed from this user session."
