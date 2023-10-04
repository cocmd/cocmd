
![Build](https://github.com/cargo-bins/cargo-binstall/workflows/Rust/badge.svg)
![](https://img.shields.io/github/v/release/Rigellute/spotify-tui?color=%23c694ff)

Cocmd is an open source tool for Developers to add Command line operations specification to their projects and 
distribute them to their community.

For example, lets say you have a lot of `git` commands you type in during the day. Use CoCMD to easily add `git` related scripts, shortcuts and automations to your terminal. 

# Getting Started

## Installation

### Step 1 - Install CoCMD
* MacOs - 
Best way to install is with brew:
```shell
brew tap cocmd/cocmd
brew install cocmd
```

* Linux - not supported yet
* Windows - not supported yet

### Step 2 - Add CoCMD to your shell

Run in terminal:
```shell
cocmd setup zsh
or 
cocmd setup bash
```

what this does is adding a loader of all aliases and automations shortcuts to your shell and lets you call any of them directly.

## Add Packages

You can start right away with `cocmd install`.
See all packages options in [Hub](https://cocmd.org/docs/packages/from_hub).

If you want to create your own read [Here](https://cocmd.org/docs/packages/package-specification)



# Showcase

## New-hire Onboarding

Onboarding new hires is a very important process in any company. It's the first impression that the new hire gets from the company and it's the first impression that the company gets from the new hire. It's also a very important process for the new hire to get to know the company and the people in it.

Read how to do it with CoCMD [Here](https://cocmd.org/docs/showcase/onboarding)

## Daily Routines

Usually, every team has a set of routines and procedures that are executed on a regular basis.
This can also be called "playbooks" or "runbooks". 

CoCMD can help you automate these routines and make them available to your team.

Read how to do it with CoCMD [Here](https://cocmd.org/docs/showcase/routines)


## Codebase CMDOps
Any project can have CMDOps (Command Line Operations). Lets say for example, your project requires some installation steps, or you want to add some shortcuts for your team or community to use.

Read how to do it with CoCMD [Here](https://cocmd.org/docs/showcase/cmdops)



# To create a release

The releases are automated via GitHub actions, using [this configuration file](https://github.com/Rigellute/spotify-tui/blob/master/.github/workflows/cd.yml).

The release is triggered by pushing a tag.

1. Bump the version in `Cargo.toml` and run the app to update the `lock` file
1. Update the "Unreleased" header for the new version in the `CHANGELOG`. Use `### Added/Fixed/Changed` headers as appropriate
1. Commit the changes and push them.
1. Create a new tag e.g. `git tag -a v0.7.0` and add the CHANGELOG to the commit body
1. Push the tag `git push --tags`
1. Wait for the build to finish on the [Actions page](https://github.com/Rigellute/spotify-tui/actions)
1. This should publish to cargo as well

### Update `brew`

1. `cd` to the [`tap` repo](https://github.com/Rigellute/homebrew-tap)
1. Run script to update the Formula `sh scripts/spotify-tui.sh $VERSION`

### Update `scoop` (Windows 10)

1. `cd` to [the `scoop` repo](https://github.com/Rigellute/scoop-bucket)
1. Run the script to update the manifest `sh scripts/spotify-tui.sh $VERSION`

## Inspiration
- Espanso - https://espanso.org/
- https://github.com/Rigellute/spotify-tui/blob/master/.github/workflows/cd.yml