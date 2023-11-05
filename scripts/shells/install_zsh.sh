#!/bin/bash
set -euo pipefail

# https://github.com/ohmyzsh/ohmyzsh/wiki/Installing-ZSH#install-and-set-up-zsh-as-default
apt install zsh
chsh -s "$(which zsh)"

exit 0