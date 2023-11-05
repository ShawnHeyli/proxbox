#!/bin/bash
set -euo pipefail

# Install fish
# https://software.opensuse.org/download.html?project=shells%3Afish%3Arelease%3A3&package=fish
echo 'deb http://download.opensuse.org/repositories/shells:/fish:/release:/3/Debian_12/ /' | sudo tee /etc/apt/sources.list.d/shells:fish:release:3.list
curl -fsSL https://download.opensuse.org/repositories/shells:fish:release:3/Debian_12/Release.key | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/shells_fish_release_3.gpg >/dev/null
sudo apt update
sudo apt install fish

# Make fish the default shell (for the current user(root))
chsh -s "$(which fish)"
