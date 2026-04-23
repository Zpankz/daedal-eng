#!/usr/bin/env bash
# daedal one-shot installer for Linux / macOS / Termux
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Hostingglobal-Tech/daedal/main/install.sh | bash
set -euo pipefail

REPO="https://github.com/Hostingglobal-Tech/daedal"
BIN_NAME="daedal"

say() { printf '\n\033[1;36m==>\033[0m %s\n' "$*"; }
err() { printf '\033[1;31m[error]\033[0m %s\n' "$*" >&2; exit 1; }

# 1) Rust toolchain
if ! command -v cargo >/dev/null 2>&1; then
  say "Rust toolchain not found. Installing with rustup."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
fi

command -v cargo >/dev/null 2>&1 || err "rustup installation failed — please retry manually (https://rustup.rs)"

# 2) Build + install daedal
say "Building and installing daedal ($(cargo --version))"
cargo install --git "$REPO" --locked

BIN_PATH="$HOME/.cargo/bin/$BIN_NAME"
[[ -x "$BIN_PATH" ]] || err "Could not find $BIN_PATH after installation."

# 3) Check PATH
if ! command -v "$BIN_NAME" >/dev/null 2>&1; then
  RC="$HOME/.bashrc"
  [[ "${SHELL##*/}" == "zsh" ]] && RC="$HOME/.zshrc"
  if ! grep -q 'cargo/bin' "$RC" 2>/dev/null; then
    printf '\nexport PATH="$HOME/.cargo/bin:$PATH"\n' >> "$RC"
    say "Added ~/.cargo/bin to PATH in $RC"
  fi
  export PATH="$HOME/.cargo/bin:$PATH"
fi

# 4) OPENAI_API_KEY
if [[ -z "${OPENAI_API_KEY:-}" ]]; then
  RC="$HOME/.bashrc"
  [[ "${SHELL##*/}" == "zsh" ]] && RC="$HOME/.zshrc"
  if grep -q '^export OPENAI_API_KEY=' "$RC" 2>/dev/null; then
    say "OPENAI_API_KEY is already present in $RC."
  else
    say "Enter your OpenAI API key (starts with sk-, hidden while typing)"
    printf "key: "
    read -rs KEY
    printf "\n"
    [[ -n "$KEY" ]] || err "Installation cannot continue with an empty key."
    printf '\nexport OPENAI_API_KEY="%s"\n' "$KEY" >> "$RC"
    export OPENAI_API_KEY="$KEY"
    say "Saved OPENAI_API_KEY to $RC"
  fi
fi

# 5) Smoke test
say "Verifying installation — generating one sample image (low quality)"
"$BIN_PATH" "a tiny friendly mascot, minimal vector style" --quality low || err "Sample request failed. Check your key and network connection."

say "Installation complete. You can now use it like this:"
printf "    %s 'prompt' [--size 1024x1024] [--quality high]\n\n" "$BIN_NAME"
