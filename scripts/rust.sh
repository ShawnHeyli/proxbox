#!/bin/bash
set -euo pipefail
# dependencies
apt install -y wget gnupg

# Easy installs
apt install -y fd-find bat ripgrep

# https://github.com/sharkdp/bat#on-ubuntu-using-apt
# Add ~/.local/bin to PATH if not already present
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
  # shellcheck disable=SC2016
  echo 'export PATH="$HOME/.local/bin:$PATH"' >>~/.bashrc
fi
ln -s "$(which batcat)" ~/.local/bin/bat

# Install eza
mkdir -p /etc/apt/keyrings
wget -qO- https://raw.githubusercontent.com/eza-community/eza/main/deb.asc | gpg --dearmor -o /etc/apt/keyrings/gierens.gpg
echo 'deb [signed-by=/etc/apt/keyrings/gierens.gpg] http://deb.gierens.de stable main' | tee /etc/apt/sources.list.d/gierens.list
chmod 644 /etc/apt/keyrings/gierens.gpg /etc/apt/sources.list.d/gierens.list
apt update
apt install -y eza

# shellcheck disable=SC1090
source ~/.bashrc
