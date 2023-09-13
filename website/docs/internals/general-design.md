---
sidebar_position: 1
---

# General Design

This is a cli python app. User installs it and out of the box has access to a default set of technologies packs.
You can add sources that contains more aliases, paths and workflows. 

## Concept of "Source"
"Source" is a location to look for packs. 
To see all sources:
```bash
cocmd sources
```
You can edit the "enabled" sources in this list. 
This list is kept in `~/.cocmd/sources.list`
You can also edit it manually.
For each source, cocmd looks for `cocmd.yaml`. If in the initial setup you configure depth to be >1 cocmd also scans internal folders accordingly. 
cocmd is just keeping inventory of what aliases and scripts to look in this uri. For example:
```yaml
name: docker
aliases: |
    alias d='docker'
    alias da='docker attach'
    alias dr='docker restart'
    alias dimg='docker images'
    alias dps='docker ps'
    alias dvol='docker volume ls'
    alias dclearimg='docker rmi $(docker images --quiet --filter "dangling=true")'
    alias dclearps='docker ps --filter status=dead --filter status=exited -aq | xargs docker rm -v'
    alias dclearvol='docker volume rm $(docker volume ls -qf dangling=true)'
    alias dc='docker-compose'
    alias dcer='docker-compose exec rails'
    alias dcerjasmine='docker-compose run --rm -e RAILS_ENV=test -p 8888:3000 rails rails jasmine'
    alias dcerspec='docker-compose run --rm -e RAILS_ENV=test rails rspec'
paths:
    - ./scripts/
    - ./bin/
automations: 
    - command: setup
      file: ./cocmd.setup.yaml
```

also create yamls for the setup automation `./cocmd.setup.yaml`:
```yaml
description: setup docker for desktop
env: debian
steps:
  - description: This will install docker on your machine
    runner: shell
    title: Install docker
    content: |
      # Update instance
      sudo apt update -y
      sudo apt upgrade -y

      # Install docker
      sudo apt install apt-transport-https ca-certificates curl software-properties-common -y
      curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
      sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu bionic stable"
      apt-cache policy docker-ce
      sudo apt install docker-ce -y

      # Automatically start Docker and Containerd on boot for non debian or ubuntu distros.
      sudo systemctl enable docker
      sudo systemctl enable containerd.service

      # Add possibility to call docker without sudo
      sudo usermod -aG docker ${USER}

      # Check is installation successful
      docker --version
  - title: Install docker-compose
    description: This will install docker-compose on your machine
    runner: shell
    content: |
      sudo curl -L "https://github.com/docker/compose/releases/download/1.26.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
      sudo chmod +x /usr/local/bin/docker-compose
      docker-compose version
```

## Source structure
A source is a uri. Currently supported is local file system and git. 
In the location we expect to see a `cocmd.yaml` that contains the information for aliases, paths and workflow. 

### Cocmd.yaml file

#### `name`

- **Type:** String
- **Description:** Specifies the name of the configuration. This could be used to identify the purpose or context of the YAML file.

#### `aliases`

- **Type:** Multiline String
- **Description:** Defines a series of command-line aliases. Each alias maps a shorter command to a more complex command, providing shortcuts for frequently used operations.

#### `paths`

- **Type:** List
- **Description:** Contains the paths to specific directories or files related to the configuration. This could include paths to scripts, binary files, or other resources that are part of the configuration.

#### `automations`

- **Type:** List of Objects
- **Description:** Specifies automation tasks that can be executed based on the configuration. Each automation object consists of:
  - `command`: The command to be executed for the automation.
  - `file`: The path to the file that contains the detailed setup or configuration for the command.

This general YAML structure provides a framework for managing aliases, paths, and automations. It can be adapted to various contexts and needs, enabling users to create shortcuts for common tasks, specify relevant paths, and define automated actions.

### Workflow file


#### `description`

- **Type:** String
- **Description:** Provides an overall description of the configuration or purpose of the YAML file. In this example, it explains that the purpose is to set up Docker for desktop.

#### `env`

- **Type:** String
- **Description:** Specifies the environment or context for the configuration. This could refer to an operating system, platform, or other context-specific information.

#### `steps`

- **Type:** List of Objects
- **Description:** Defines a series of sequential steps or tasks to be performed. Each step object consists of:
  - `description`: A brief explanation of what the step accomplishes.
  - `runner`: Specifies the type of runner or execution environment for the step (e.g., shell, Python, etc.).
  - `title`: A title or name for the step, often used for human-readable identification.
  - `content`: A multiline string containing the actual commands or scripts to be executed for the step. This can include shell commands, scripts, or other executable content.

The given YAML structure serves as a blueprint for automating a series of tasks or steps. It provides a clear and organized way to define what needs to be done, in what order, and how each step should be executed. This can be adapted for various purposes such as software installation, system configuration, workflow automation, and more.

## The Default source
Out of the box, upon cocmd installation there is a "default" source configured which contains default technologies for you to add. 

## Cocmd execution flow
Assuming cocmd is setup and configured. 
Upon finishing a command that should change any of the sources content
like
- adding a new source
- cocmd setup
- running `cocmd refresh`
cocmd scans the sources again and updates your rc file for the shell you configured. 
What is added to the shell rc file?
```
eval "$(cocmd profile-loader)"
```

this addes aliases, automations and paths to the environment.


