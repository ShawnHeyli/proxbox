#!/bin/bash
set -ueo pipefail

# Ask if they want to use a script or a git repo for a chezmoi source or nothing at all
# If they choose a script, ask if they want to use a script from the repo or a script from a url
# If they choose a git repo, ask for the url
# If they choose nothing, exit

