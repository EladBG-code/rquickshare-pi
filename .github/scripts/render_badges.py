#!/usr/bin/env python3
from __future__ import annotations

import argparse
import html
from pathlib import Path


def text_width(value: str) -> int:
    return max(28, (len(value) * 7) + 10)


def render_badge(label: str, message: str, label_color: str, message_color: str) -> str:
    label_width = text_width(label)
    message_width = text_width(message)
    total_width = label_width + message_width
    label_x = label_width / 2
    message_x = label_width + (message_width / 2)
    aria = html.escape(f"{label}: {message}", quote=True)

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="20" role="img" aria-label="{aria}">
  <title>{aria}</title>
  <linearGradient id="s" x2="0" y2="100%">
    <stop offset="0" stop-color="#fff" stop-opacity=".08"/>
    <stop offset="1" stop-color="#000" stop-opacity=".08"/>
  </linearGradient>
  <clipPath id="r">
    <rect width="{total_width}" height="20" rx="0" fill="#fff"/>
  </clipPath>
  <g clip-path="url(#r)">
    <rect width="{label_width}" height="20" fill="{label_color}"/>
    <rect x="{label_width}" width="{message_width}" height="20" fill="{message_color}"/>
    <rect width="{total_width}" height="20" fill="url(#s)"/>
  </g>
  <g fill="#fff" text-anchor="middle" font-family="Verdana,DejaVu Sans,sans-serif" font-size="11">
    <text x="{label_x}" y="15" fill="#010101" fill-opacity=".3">{html.escape(label)}</text>
    <text x="{label_x}" y="14">{html.escape(label)}</text>
    <text x="{message_x}" y="15" fill="#010101" fill-opacity=".3">{html.escape(message)}</text>
    <text x="{message_x}" y="14">{html.escape(message)}</text>
  </g>
</svg>
"""


def write_badge(path: Path, label: str, message: str, color: str) -> None:
    path.write_text(render_badge(label, message, "#3b363d", color), encoding="utf-8")


def compact_number(raw_value: str) -> str:
    value = int(raw_value)
    if value >= 1_000_000:
        return f"{value / 1_000_000:.1f}M".replace(".0M", "M")
    if value >= 1_000:
        return f"{value / 1_000:.1f}k".replace(".0k", "k")
    return str(value)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", required=True)
    parser.add_argument("--latest", required=True)
    parser.add_argument("--stars", required=True)
    parser.add_argument("--downloads", required=True)
    args = parser.parse_args()

    output = Path(args.output)
    output.mkdir(parents=True, exist_ok=True)

    write_badge(output / "latest-release.svg", "latest", args.latest, "#2f6df6")
    write_badge(output / "stars.svg", "stars", compact_number(args.stars), "#f6c343")
    write_badge(output / "downloads.svg", "downloads", compact_number(args.downloads), "#1f9d55")


if __name__ == "__main__":
    main()
