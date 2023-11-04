# https://starship.rs/guide/#%F0%9F%9A%80-installation
curl -sS https://starship.rs/install.sh | sh

echo 'eval "$(starship init bash)"' >> ~/.bashrc

# https://starship.rs/config/
mkdir -p ~/.config && touch ~/.config/starship.toml
starship preset $1 -o ~/.config/starship.toml