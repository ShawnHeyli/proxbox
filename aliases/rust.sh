aliases="
# Rust alternatives aliases from proxbox
alias ld='eza -lD' # lists only directories (no files)
alias lf='eza -lF --color=always | grep -v /' # lists only files
alias ll='eza -al --group-directories-first' # lists everything with directories first
alias ls='eza -l --color=always' # ls with color
alias la='eza -al --color=always' # lists everything

alias cat='bat --pager=never --style=numbers,changes --color=always' # cat with line numbers and git changes
"

echo $aliases >> ~/.bashrc