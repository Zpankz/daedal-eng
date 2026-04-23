# imagen

Tiny Rust CLI for OpenAI **`gpt-image-2`** image generation.
Single static binary, no Python, no Node.js.

[![Rust](https://img.shields.io/badge/Rust-stable-orange)](https://rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

## Why

Most OpenAI image clients are heavyweight Node.js / Python SDKs.
This is a **single ~2.3 MB static binary** that does one thing: send a prompt
to OpenAI's official `/v1/images/generations` endpoint with `gpt-image-2`
hardcoded, decode the base64 PNG, write the file.

- No SDK dependency
- Direct REST call (reqwest + rustls)
- Cross-platform: Linux, macOS, Windows, Android (Termux)
- Smart default output paths per environment

## Requirements

- `OPENAI_API_KEY` environment variable (account with `gpt-image-2` access)

## Install

### Option A — Claude Code (one-liner)

Tell Claude Code:

> Install the imagen skill from `github.com/Hostingglobal-Tech/imagen`, build with cargo, drop binary in `~/bin`.

### Option B — cargo install (direct)

```bash
cargo install --git https://github.com/Hostingglobal-Tech/imagen --locked
```

### Option C — build from source

```bash
git clone https://github.com/Hostingglobal-Tech/imagen
cd imagen
cargo build --release
install -m 755 target/release/imagen ~/bin/imagen
```

Set API key (bash):

```bash
echo 'export OPENAI_API_KEY="sk-..."' >> ~/.bashrc
```

## Usage

```bash
imagen "a red cube on white background"
imagen "a blue cat sitting on moon, oil painting" --quality high
imagen "korean hanok with cherry blossoms" --size 1024x1536 -o hanok.png
imagen "3 variants" -n 3 --quality auto
imagen "for scripts" --quiet -o out.png   # prints only file path
```

### Options

| Flag | Values | Default |
|---|---|---|
| `--size` | `1024x1024` · `1024x1536` · `1536x1024` · `auto` | `1024x1024` |
| `--quality` | `low` · `medium` · `high` · `auto` | `auto` |
| `-n` | `1..=10` | `1` |
| `-o, --out` | any path | see below |
| `--quiet` | — | off |

### Default Output Paths

If `--out` is not given, the file goes to:

| Platform | Path |
|---|---|
| Termux / Android | `/sdcard/DCIM/imagen-<epoch>.png` (auto MediaScan) |
| WSL | `/mnt/c/Users/<you>/Pictures/imagen/` |
| Windows | `%USERPROFILE%\Pictures\imagen\` |
| Linux | `$HOME/Pictures/imagen/` |
| Override | export `IMAGEN_OUT_DIR=/any/path` |

The output directory is created automatically.

## Examples

Prompt: *"실사풍 한국 전통 한옥 마당에 벚꽃이 만발한 봄날 오후, 따뜻한 햇빛, 기와 지붕 디테일 정교, 고해상도 사진"*
Size: `1024x1536`, quality: `high`, duration: 106 s, output: 3.2 MB

![Hanok with cherry blossoms](examples/hanok-blossom.png)

Prompt: *"a futuristic seoul skyline at sunset, photorealistic"*
Size: `1024x1024`, quality: `low`, duration: ~45 s, output: 1.7 MB

![Seoul skyline](examples/seoul-skyline.png)

## Model

**`gpt-image-2`** is hardcoded. No CLI flag to override.
If you need a different model, fork and change the `MODEL` constant in `src/main.rs`.

Pinned variant resolved via `/v1/models`: `gpt-image-2-2026-04-21` (at time of writing).

## Cost

Usage is returned in stderr after each call:

```
[imagen] usage: {"total_tokens":211, ...}
```

Refer to OpenAI pricing for `gpt-image-2`.

## Security

- No API keys in source. Read from `OPENAI_API_KEY` env only.
- TLS via `rustls` (no OpenSSL).
- No telemetry, no analytics, no crash reporting.
- Outbound traffic: `api.openai.com` only.

## License

MIT — see [LICENSE](LICENSE).

## Contributing

Keep the binary small and the code boring. No feature creep, no plugin system.
One file, one purpose.
