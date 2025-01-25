################################################################################
# GITCONFIG REFERENCE GUIDE
# Purpose: Quick reference for setting up Git configuration
# Usage: Copy relevant sections to your ~/.gitconfig or .git/config
################################################################################

# Basic User Settings
# ------------------
#[user]
#    name = [GitHub/Repository User Name]
#    email = [GitHub/Repository User Email]
#    signingkey = [GPG Key ID]

# Commit Signing
# -------------
#[commit]
#    gpgsign = true

# Core Settings
# ------------
#[core]
#    editor = code --wait
#    autocrlf = input
#    whitespace = fix,-indent-with-non-tab,trailing-space,cr-at-eol
#    excludesfile = ~/.gitignore_global
#    eol = lf
#    safecrlf = false
#    filemode = false
#    credentialHelper = cache --timeout=3600

# UI Colors
# --------
#[color]
#    ui = auto
#    branch = auto
#    diff = auto
#    status = auto

# Useful Aliases
# -------------
#[alias]
    # Status & Information
#    s = status -sb
#    alias = config --get-regexp ^alias\\.
#    whoami = !git config user.name && git config user.email

    # Logging & History
#    lg = log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit --date=relative
#    hist = log --pretty=format:'%h %ad | %s%d [%an]' --graph --date=short
#    last = log -1 HEAD

    # Branching & Checkout
#    b = branch
#    bd = branch -D
#    cb = checkout -b
#    co = checkout
#    main = !git checkout main && git pull

    # Changes & Staging
#    d = diff
#    dc = diff --cached
#    changes = diff --name-status
#    untrack = rm --cached
#    unstage = reset HEAD --

    # Commits
#    c = commit
#    cm = commit -m
#    ca = commit --amend
#    undo = reset --soft HEAD^

    # Sync & Remote
#    p = push
#    pf = push --force-with-lease
#    pl = pull
#    fetch-all = fetch --all --prune

    # Stash Operations
#    save = stash save
#    pop = stash pop
#    stashes = stash list

    # Cleanup & Maintenance
#    cleanup = !git remote prune origin && git gc && git clean -df
#    fresh = !git reset --hard HEAD && git clean -df

    # Quick Operations
#    a = add
#    aa = add --all
#    br = branch -vv
#    cp = cherry-pick
#    st = status -sb

    # Smart Operations
#    amend = commit --amend --no-edit
#    uncommit = reset --soft HEAD^
#    unadd = reset HEAD

    # Branch Management
#    gone = ! git fetch -p && git for-each-ref --format '%(refname:short) %(upstream:track)' | awk '$2 == \"[gone]\" {print $1}' | xargs -r git branch -D
#    sync = !git fetch -p && git pull --rebase

# Pull Settings
# ------------
#[pull]
#    rebase = true

# Branch Settings
# -------------
#[init]
#    defaultBranch = main

# Push Settings
# ------------
#[push]
#    default = current
#    followTags = true

# Merge Settings
# -------------
#[merge]
#    tool = vscode
#    conflictstyle = diff3
#    ff = false

#[mergetool "vscode"]
#    cmd = code --wait $MERGED

# Diff Settings
# ------------
#[diff]
#    tool = vscode
#    wordRegex = .
#    renames = copies
#    algorithm = histogram

#[difftool "vscode"]
#    cmd = code --wait --diff $LOCAL $REMOTE

# Fetch Settings
# -------------
#[fetch]
#    prune = true

# Rebase Settings
# --------------
#[rebase]
#    autoStash = true

# Help Settings
# ------------
#[help]
#    autocorrect = 20

# URL Settings
# -----------
#[url "git@github.com:"]
#    insteadOf = https://github.com/