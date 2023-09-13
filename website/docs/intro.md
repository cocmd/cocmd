---
sidebar_position: 1
---

# Getting Started

Cocmd is an open source tool for Developers to add aliases and scripts to their Bash terminal commands (or any other Shell - zsh, fish etc).

For example, lets say you have a lot of `git` commands you type in during the day. Use CoCMD to easily add `git` related scripts, shortcuts and automations to your terminal. 



### 1. Install
First, please install cocmd with pip or poetry:

```bash
pip install cocmd
```

### 2. Setup
Second, we need some information for the first setup:
```bash
cocmd setup
```

You will get several questions to make CoCMD work for you:
1. what is your shell
  - this is important because we setup aliases and PATH in the rc file 
2. scan depth (default is *1*)
  - for each source, how deep to scan for `cocmd.yaml` file.

### 3. Add Tech Packs
Add your favorite technologies
```bash
cocmd add tech <tech>
```
You will get a list of possible technologies to add to the cmd.
