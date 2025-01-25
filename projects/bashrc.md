################################################################################
# BASHRC REFERENCE GUIDE
# Purpose: Quick reference for setting up Bash environment
# Usage: Copy relevant sections to your ~/.bashrc
# Note: Works for Git Bash (Windows) and Linux environments
################################################################################

# Path Configuration
# -----------------
#PATH=$PATH:/c/Go/bin

# NVM Configuration
# ----------------
#export NVM_DIR="$HOME/.nvm"
#[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
#[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# Bun Configuration
# ----------------
#export BUN_INSTALL="$HOME/.bun"
#export PATH="$BUN_INSTALL/bin:$PATH"

# Git Branch Detection
# ------------------
#parse_git_branch() {
#    git rev-parse --git-dir > /dev/null 2>&1 && git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/ [\1]/'
#}

# Prompt Configuration
# ------------------
#PS1='\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[33m\]$(parse_git_branch)\[\033[00m\]\$ '

# Environment Settings
# ------------------
#export EDITOR='code --wait'
#export VISUAL='code --wait'
#export HISTSIZE=10000
#export HISTFILESIZE=20000
#export HISTCONTROL=ignoreboth:erasedups

# Help Function
# ------------
#function aliases() {
#    echo "��� Available Aliases and Functions:"
#    # ...existing help text...
#}
#alias help='aliases'

# Directory Navigation
# ------------------
#alias ..='cd ..'
#alias ...='cd ../..'
#alias ....='cd ../../..'
#alias .....='cd ../../../..'
#alias ~='cd ~'
#alias -- -='cd -'

# Enhanced Listing
# --------------
#alias ls='ls -F --color=auto'
#alias ll='ls -lah'
#alias la='ls -A'
#alias l='ls -CF'
#alias dir='dir --color=auto'

# Directory Shortcuts
# -----------------
#alias dev='cd /c/dev-environment'
#alias proj='cd /c/Projects'

# Project Workflow Helpers
# ----------------------
#alias serve='python -m http.server'
#alias ports='netstat -tulanp'
#alias myip='curl http://ipecho.net/plain; echo'

# Search Functions
# --------------
#function f() { find . -iname "*$1*" ${2:-.}; }
#function r() { grep "$1" ${2:-.} -R; }

# Directory Creation
# ----------------
#function mkcd() { mkdir -p "$@" && cd "$_"; }

# Archive Extraction
# ----------------
#function extract() {
#    # ...existing extract function...
#}

# Project Setup
# -----------
#function new-project() {
#    # ...existing new-project function...
#}

# Command Timer
# -----------
#function timer() {
#    # ...existing timer function...
#}

# Directory Bookmarks
# -----------------
#if [ ! -f ~/.dirs ]; then
#    touch ~/.dirs
#fi
#alias mark='pwd >> ~/.dirs'
#alias marks='cat ~/.dirs'
#alias jump='cd $(cat ~/.dirs | fzf)'

# Git Integration
# -------------
#source /etc/profile.d/git-sdk.sh >/dev/null 2>&1 || true

# NVM Setup
# --------
#if [ -d "$HOME/.nvm" ]; then
#    export NVM_DIR="$HOME/.nvm"
#    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" --no-use
#    [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
#fi

# Welcome Message
# -------------
#echo ""
#echo "  🚀 Terminal Initialized! 🎯"
#echo "  ⏰ Current time: $(date '+%Y-%m-%d %H:%M:%S')"
#echo "  👋 Welcome back, ${USER:-$(whoami)}!"
#echo "  📂 Working directory: $(pwd)"
#echo "  💡 Type 'help' to see available commands"
#echo ""