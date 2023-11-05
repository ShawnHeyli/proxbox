#!/bin/bash
set -euo pipefail

# depedencies
apt install -y wget sudo

# Get latest nushell release
INSTALL_LOCATION=/usr/local/bin

cd /tmp
wget https://github.com/nushell/nushell/releases/latest/download/nu-0.86.0-x86_64-linux-musl-full.tar.gz -O nu.tar.gz
tar -xvf nu.tar.gz
rm nu.tar.gz
mv nu-* nu
mv nu/nu $INSTALL_LOCATION
rm -r nu

chsh -s "$(which nu)"

# Get default config on our own to skip nu first prompt
mkdir -p ~/.config/nushell
wget https://raw.githubusercontent.com/nushell/nushell/main/crates/nu-utils/src/sample_config/default_env.nu -O ~/.config/nushell/env.nu
wget https://raw.githubusercontent.com/nushell/nushell/main/crates/nu-utils/src/sample_config/default_config.nu -O ~/.config/nushell/config.nu

# Disable welcome message
sed -i 's/show_banner: true/show_banner: false/g' ~/.config/nushell/config.nu

# Add nushell to /etc/shells if not already present
if [[ "$(cat /etc/shells)" != *$(which nu)* ]]; then
    which nu | sudo tee -a /etc/shells
fi
