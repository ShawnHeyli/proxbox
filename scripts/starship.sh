#!/bin/bash
set -euo pipefail

# dependencies
apt install -y curl

# https://starship.rs/guide/#%F0%9F%9A%80-installation
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /usr/local/bin --force
echo 'eval "$(starship init bash)"' >>~/.bashrc

# https://starship.rs/config/
mkdir -p ~/.config && touch ~/.config/starship.toml
starship preset $1 -o ~/.config/starship.toml

# shellcheck disable=SC1090
source ~/.bashrc
