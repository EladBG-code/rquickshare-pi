# Pi-Apps Submission Notes

This repository contains a ready-to-copy Pi-Apps app folder at:

```text
packaging/pi-apps/RQuickShare Pi
```

## Eligibility Check

RQuickShare Pi fits the current Pi-Apps app guidelines:

- It is useful for Raspberry Pi desktop users.
- It is a GUI desktop app, not a headless/server-only tool.
- It is legal open-source software under GPL-3.0.
- It is appropriate for all ages.
- It is ARM64-only, which Pi-Apps issue #185 says is acceptable as of the 2026 update.
- It has no malicious behavior and installs from the public GitHub release asset.

## Pi-Apps Files Prepared

The app folder includes:

- `icon-64.png`
- `icon-24.png`
- `website`
- `description`
- `credits`
- `install-64`
- `uninstall`

The category entry to add to Pi-Apps `etc/categories` is:

```text
RQuickShare Pi|Internet/Download & Upload
```

## Why This Uses `install-64`

RQuickShare Pi currently targets Raspberry Pi OS 64-bit on ARM64 only. Pi-Apps supports architecture-specific scripts, so this app uses `install-64` and does not provide an `install-32` script.

## Release Update Checklist

When publishing a new RQuickShare Pi release, update this line in `packaging/pi-apps/RQuickShare Pi/install-64`:

```bash
version="0.0.2-alpha"
```

Then verify the `.deb` asset name still matches:

```bash
RQuickShare.Pi_${version}_arm64.deb
```
