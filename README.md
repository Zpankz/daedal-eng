# daedal

Small Rust CLI for generating images with OpenAI **`gpt-image-2`**.
Single static binary. No Python or Node.js required.

[![Rust](https://img.shields.io/badge/Rust-stable-orange)](https://rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

![Hanok courtyard in blossom](examples/hanok-blossom.png)

## Name

Named after **Daedalus**, the craftsman of Greek mythology — designer of the labyrinth and maker of wax wings.
The English adjective `daedal` also means “skillfully made” or “intricate.”

---

## 🚀 Easiest install — one Claude Code prompt

You do not need to know Rust, cargo, or shell environment variables. In [Claude Code](https://www.claude.com/claude-code), **copy and paste the paragraph below exactly as written**:

> Please install the daedal CLI. The repository is `https://github.com/Hostingglobal-Tech/daedal`.
>
> Please do the following automatically in this order, without asking me questions:
> 1. Install the Rust toolchain (`rustup`) if it is missing.
> 2. Run `cargo install --git https://github.com/Hostingglobal-Tech/daedal --locked`.
> 3. If `~/.cargo/bin` is not in `PATH`, add it to `~/.bashrc` (or `~/.zshrc`).
> 4. Prompt me to paste my OpenAI API key, then add `OPENAI_API_KEY` to the shell rc file.
> 5. Generate one image with `daedal "a small red apple on white table" --quality low` to verify it works.
> 6. Tell me the installed binary path and the test image path.

Claude Code handles the full setup. Just have your OpenAI API key ready in advance ([create one here](https://platform.openai.com/api-keys)).

### No Claude Code? Use the one-line install script instead

In a Linux / macOS / Termux terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/Hostingglobal-Tech/daedal/main/install.sh | bash
```

What it does automatically:
- Installs the Rust toolchain with `rustup` if needed
- Builds and installs daedal with `cargo install`
- Adds `~/.cargo/bin` to your shell rc file if it is missing from `PATH`
- Prompts for your OpenAI API key and saves it to the rc file
- Generates one sample image to verify everything works

---

## Usage

```bash
daedal "red cube on white background"
daedal "blue cat sitting on the moon, oil painting style" --quality high
daedal "hanok courtyard with cherry blossoms" --size 1024x1536 -o hanok.png
daedal "3 logo concepts" -n 3
daedal "for scripting" --quiet -o out.png   # print only the file path to stdout
```

### Options

| Flag | Value | Default |
|---|---|---|
| `--size` | `1024x1024` · `1024x1536` · `1536x1024` · `auto` | `1024x1024` |
| `--quality` | `low` · `medium` · `high` · `auto` | `auto` |
| `-n` | 1..=10 images | `1` |
| `-o, --out` | Save path | See the table below |
| `--quiet` | — | off |

### Default output location

If you omit `--out`:

| Platform | Path |
|---|---|
| Android (Termux) | `/sdcard/DCIM/daedal-<epoch>.png` (auto-indexed into the gallery) |
| Windows | `%USERPROFILE%\Pictures\daedal\` |
| macOS / Linux | `$HOME/Pictures/daedal/` |
| Custom via env var | `export DAEDAL_OUT_DIR=/your/preferred/path` |

The directory is created automatically if it does not exist.

### Examples

Prompt: *"a futuristic Seoul skyline at sunset, photorealistic"* · `1024x1024` · `low` quality

![Seoul skyline](examples/seoul-skyline.png)

Prompt: *"photorealistic traditional Korean hanok courtyard in spring, cherry blossoms in full bloom, warm afternoon sunlight, detailed tiled roof, high-resolution photo"* · `1024x1536` · `high` quality

![Hanok courtyard in blossom](examples/hanok-blossom.png)

### Korean text rendering

Starting with `gpt-image-2`, **Korean signage and typography** render much more reliably. Earlier generations (DALL-E 3 and `gpt-image-1`) often produced broken syllables or incorrect Hanja-like glyphs, while the newer generation is much better at serif, sans-serif, brush, and calligraphic Korean text.

Useful examples:
```bash
daedal "Korean traditional restaurant sign reading 'Matjip 1998', brush lettering, engraved wood plaque, low lighting"
daedal "Seoul subway sign reading 'Gangnam Station Exit 1 -> toward Samseong Station', white text on blue background"
daedal "Spring flower festival poster reading 'April Cherry Blossom Festival', Korean-style calligraphy plus date '2026.04.10-20'" --size 1024x1536
```

This is especially useful for posters, storefront signs, and UI mockups that need Korean text.

---

## Manual installation (advanced users)

To install it manually without Claude Code:

### A. cargo install

```bash
# 1) Install Rust (skip if already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2) Build and install daedal (creates ~/.cargo/bin/daedal)
cargo install --git https://github.com/Hostingglobal-Tech/daedal --locked

# 3) Set your OpenAI API key
echo 'export OPENAI_API_KEY="sk-..."' >> ~/.bashrc
source ~/.bashrc

# 4) Test it
daedal "a cute red panda" --quality low
```

### B. Build from source

```bash
git clone https://github.com/Hostingglobal-Tech/daedal
cd daedal
cargo build --release
cp target/release/daedal ~/.local/bin/   # or any directory already in PATH
```

### Windows PowerShell

```powershell
# Install Rust: download and run rustup-init.exe from https://rustup.rs
cargo install --git https://github.com/Hostingglobal-Tech/daedal --locked
setx OPENAI_API_KEY "sk-..."
# Open a new PowerShell window for the setx value to take effect
daedal "a red cube on white"
```

---

## Requirements

- Rust stable toolchain (for building)
- `OPENAI_API_KEY` environment variable — an OpenAI account with access to `gpt-image-2`

---

## Model

**`gpt-image-2`** is hard-coded in the source. There is no CLI flag to change it.
If you want a different model, fork the repo and edit the `MODEL` constant in `src/main.rs`.

## Cost

Each request prints usage information to stderr:

```
[daedal] usage: {"total_tokens":211, ...}
```

For detailed pricing, see the `gpt-image-2` entry on [OpenAI pricing](https://openai.com/api/pricing/).

## Security

- No API keys are hard-coded in the source. The program only reads `OPENAI_API_KEY`.
- TLS uses `rustls` (no OpenSSL dependency).
- No telemetry, analytics, or error reporting.
- Only one external network destination: `api.openai.com`.

## License

MIT — see [LICENSE](LICENSE).

## Contribution

Keep the binary small and the code boring. No feature bloat and no plugin system.
One file, one purpose.
