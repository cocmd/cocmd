

import pprint
from cocmd_cli_app.utils import io
from cocmd_cli_app.core.models.script_model import *



data_osx = ScriptModel(
    name="devmachine",
    title="setup osx dev machine",
    description="setup",
    spec=SpecModel(
        alias="osx",
        globals=[
            
        ],
        variations=[
        StepsModel(
            env=OS.OSX,
            steps=[
                StepModel(
                    title="install homebrew",
                    description="---",
                    runner=StepRunnerType.SHELL,
                    content="""
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

echo '# Set PATH, MANPATH, etc., for Homebrew.' >> ~.zprofile
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile



echo '# Set PATH, MANPATH, etc., for Homebrew.' >> ~.bash_profile
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.bash_profile


"""
                ),
                StepModel(
                    title="install basics (curl, wget, unzip, git etc)",
                    description="---",
                    runner=StepRunnerType.SHELL,
                    content="""
echo '💻 Installing: basic'
brew install curl
brew install wget
brew install unzip
brew install tree
brew install neofetch
brew install wxwidgets
brew install libxslt fop

echo '💻 Installing: CLI tools'
brew install git


echo '💻 Configuring: git'
git config --global user.name "$MY_NAME"
git config --global user.email "$MY_EMAIL"

"""
                ),
                
                StepModel(
                    title="install python essentials",
                    description="---",
                    runner=StepRunnerType.SHELL,
                    content="""

if ! pip3 help > /dev/null; then
  echo "installing python & pip ..."
  #brew install python2
  brew install python3
fi


wget https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-x86_64.sh -O ~/miniconda.sh
bash ~/miniconda.sh -b -p $HOME/miniconda


"""
                ),
                
            ]),
    ])
)

io.YamlIO.to_file('./cocmd_cli_app/resources/demo/osx/scripts/devmachine.yaml', data_osx)


data_zsh_setup = ScriptModel(
    name="setup",
    title="Setup zsh",
    spec=SpecModel(
        globals=[
            
        ],
        variations=[
        StepsModel(
            env=OS.LINUX,
            steps=[
                StepModel(
                    title="install-zsh",
                    description="install and setup zsh on your computer",
                    runner=StepRunnerType.PYTHON,
                    content="""
'''
### Installation script ###
https://gist.github.com/headrockz/8959b18d7ae69bb45e4b7421ace54fa0
Shell -> zsh, oh-my-zsh
Theme -> spaceship
plugins -> git docker python zsh-syntax-highlighting zsh-autosuggstions web-search ssh-agent
'''

import os


def installZsh():
    print("Installing zsh, git and curl")
    os.system('sudo apt install zsh git curl -y && echo "ZSH successfully installed"')
    print()

def installOhMyZsh():
    print("installing oh-my-zsh")
    os.system('git clone https://github.com/ohmyzsh/ohmyzsh.git ~/.oh-my-zsh && echo "oh-my-zsh successfully installed"')
    print()


def installZshSyntaxHighlighting():
    print("Installing zsh-syntax-highlighting")
    os.system(r'git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting && echo "zsh-syntax-highlighting successfully installed"')
    print()

def installZshAutosuggestions():
    print("Installing zsh-autosuggestions")
    os.system(r'git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions && echo "zsh-autosuggestions successfully installed"')
    print()



def installSpaceShip():
    print("Installing spaceship theme")
    os.system(r'git clone https://github.com/spaceship-prompt/spaceship-prompt.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/themes/spaceship-prompt --depth=1')
    print("Creating symbolic link")
    os.system(r'ln -s ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/themes/spaceship-prompt/spaceship.zsh-theme ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/themes/spaceship.zsh-theme')
    print()

def zshrc():
    print("Cloning my .zshrc")
    os.system('git clone https://gist.github.com/28f52538d94785d737d852f938da7fde.git')
    os.system('mv 28f52538d94785d737d852f938da7fde/.zshrc ~/')
    os.system('sudo usermod --shell $(which zsh) $USER')

installZsh()
installOhMyZsh()
installZshSyntaxHighlighting()
installZshAutosuggestions()
installSpaceShip()
zshrc()

print('Close and reopen your terminal')
print('Have a nice day!')                    
                    """
                ),
            ]),

        StepsModel(
                env=OS.OSX,
                steps=[
                    StepModel(
                        title="install zsh",
                        description="install  zsh on your computer",
                        runner=StepRunnerType.SHELL,
                        content="""

    echo '💻 Installing: zsh'
    brew install zsh
    chsh -s $(which zsh)
    curl -sS https://starship.rs/install.sh | sh
    touch ~/.zshrc
                        """
                    ),
                    StepModel(
                        title="configure zsh",
                        description="configure  zsh on your computer",
                        runner=StepRunnerType.SHELL,
                        content="""
    echo '💻 Installing: antigen'
    mkdir ~/.antigen
    curl -L git.io/antigen > ~/.antigen/config.zsh

    echo '💻 Configuring: ZSH'
    echo '
    # antigen 
    source ~/.antigen/config.zsh
    antigen use oh-my-zsh
    antigen bundle git
    antigen bundle github
    antigen bundle git-flow
    antigen bundle docker
    antigen bundle docker-compose
    antigen bundle asdf
    antigen bundle node
    antigen bundle npm
    antigen bundle elixir
    antigen bundle zsh-users/zsh-syntax-highlighting
    antigen bundle zsh-users/zsh-autosuggestions
    antigen bundle zsh-users/zsh-completions
    antigen apply
    # spaceship theme
    eval "$(starship init zsh)"
    ' >> ~/.zshrc"""
                    )])
            ]
                
    )
)

io.YamlIO.to_file('./cocmd_cli_app/resources/demo/zsh/scripts/setup.yaml', data_zsh_setup)



data_onboarding = ScriptModel(
    name="onboarding",
    title="New Developer onboarding ",
    description="Welcome to the team!",
    spec=SpecModel(
        globals=[
            StepGlobalModel(
                    title="Welcome",
                    description="",
                    runner=StepRunnerType.MARKDOWN,
                    content="""
# Welcome to Team1
## happy to have you on the team!

This will take you through the onboarding

We value the talents and ideas of everyone on our team, especially our new hires. We can’t wait to see what you’ll make happen

Welcome! We choose our new team members carefully and we’re proud to welcome you, with all of your talents and ideas and faults. Yes, faults — making mistakes is an important part of growth, and we hope you’ll never worry about trying to be perfect on the job. We sure aren’t

                    """,
                    id="welcome_md"
                )
        ],
        variations=[
        StepsModel(
            env=OS.ANY,
            steps=[
               StepRefModel(ref="welcome_md"),
               StepRefModel(ref="osx.devmachine"), 
               StepRefModel(ref="zsh.setup"), 
               StepRefModel(ref="docker.setup"), 
               StepRefModel(ref="k8s.setup"),
               StepRefModel(ref="awscli.setup"),
            ]),
    ])
)

io.YamlIO.to_file('./cocmd_cli_app/resources/demo/team1/scripts/onboarding.yaml', data_onboarding)



data_docker_clean = ScriptModel(
    name="clean",
    title="clean docker script",
    description="clean clean clean",
    spec=SpecModel(
        alias="dkclean",
        globals=[
            
        ],
        variations=[
        StepsModel(
            env=OS.ANY,
            steps=[
                StepModel(
                    title="clean docker",
                    description="This will install docker on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
#!/bin/bash

# options:
# remove stopped containers and untagged images
#   $ dkcleanup 
# remove all stopped|running containers and untagged images
#   $ dkcleanup --reset
# remove containers|images|tags matching {repository|image|repository\image|tag|image:tag}
# pattern and untagged images
#   $ dkcleanup --purge {image}
# everything
#   $ dkcleanup --nuclear

echo "moshe"
echo "$1"

if [ "$1" == "--reset" ]; then
    # Remove all containers regardless of state
    docker rm -vf $(docker ps -a -q) 2>/dev/null || echo "No more containers to remove."
elif [ "$1" == "--purge" ]; then
    # Attempt to remove running containers that are using the images we're trying to purge first.
    (docker rm -vf $(docker ps -a | grep "$2/\|/$2 \| $2 \|:$2\|$2-\|$2:\|$2_" | awk '{print $1}') 2>/dev/null || echo "No containers using the \"$2\" image, continuing purge.") &&\
    # Remove all images matching arg given after "--purge"
    docker rmi $(docker images | grep "$2/\|/$2 \| $2 \|$2 \|$2-\|$2_" | awk '{print $1 ":" $2}') 2>/dev/null || echo "No images matching \"$2\" to purge."
else
    # This alternate only removes "stopped" containers
    docker rm -vf $(docker ps -a | grep "Exited" | awk '{print $1}') 2>/dev/null || echo "No stopped containers to remove."
fi

if [ "$1" == "--nuclear" ]; then
    docker rm -vf $(docker ps -a -q) 2>/dev/null || echo "No more containers to remove."
    docker rmi $(docker images -q) 2>/dev/null || echo "No more images to remove."
else
    # Always remove untagged images
    docker rmi $(docker images | grep "<none>" | awk '{print $3}') 2>/dev/null || echo "No untagged images to delete."
fi

exit 0
"""
                ),
                
            ]),
    ])
)

io.YamlIO.to_file('./cocmd_cli_app/resources/demo/docker/scripts/clean.yaml', data_docker_clean)

data_docker = ScriptModel(
    name="setup",
    title="Setup Docker ",
    description="setup docker for desktop",
    spec=SpecModel(
        globals=[
            
        ],
        variations=[
        StepsModel(
            env=OS.LINUX,
            label="debian",
            steps=[
                StepModel(
                    title="Install docker",
                    description="This will install docker on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
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

                        # Check is instalation successful
                        docker --version
                        """
                ),
                StepModel(
                    title="Install docker-compose",
                    description="This will install docker-compose on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
                        sudo curl -L "https://github.com/docker/compose/releases/download/1.26.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
                        sudo chmod +x /usr/local/bin/docker-compose
                        docker-compose version
                        """
                ),
            ]),

        StepsModel(
            env=OS.OSX,
            label=">10.x.x",
            depends=[
                "installed:brew",
            ],
            steps=[
                StepModel(
                    title="Install docker",
                    description="This will install docker on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
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

                        # Check is instalation successful
                        docker --version
                        """
                ),
                StepModel(
                    title="Install docker-compose",
                    description="This will install docker-compose on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
                        sudo curl -L "https://github.com/docker/compose/releases/download/1.26.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
                        sudo chmod +x /usr/local/bin/docker-compose
                        docker-compose version
                        """
                ),
            ]),
    ])
)

io.YamlIO.to_file('./cocmd_cli_app/resources/demo/docker/scripts/setup.yaml', data_docker)


data_awscli = ScriptModel(
    name="setup",
    title="Setup AwsCli ",
    description="setup awscli for desktop",
    spec=SpecModel(
        globals=[
            
        ],
        variations=[
        StepsModel(
            env=OS.LINUX,
            label="debian",
            steps=[
                StepModel(
                    title="Install awscli",
                    description="This will install awscli on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
                        curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
                        unzip awscliv2.zip
                        sudo ./aws/install
                        aws --version
                        rm -rf awscliv2.zip aws
                        """
                ),
                StepModel(
                    title="Configure Aws credentials and environment",
                    description="",
                    runner=StepRunnerType.SHELL,
                    content="""
                    aws configure
                    """
                )
            ]),

        StepsModel(
            env=OS.OSX,
            label=">10.x.x",
            depends=[
                "installed:brew",
            ],
            steps=[
                StepModel(
                    title="Install awscli",
                    description="This will install awscli on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
                        curl "https://awscli.amazonaws.com/AWSCLIV2.pkg" -o "AWSCLIV2.pkg"
                        sudo installer -pkg AWSCLIV2.pkg -target /
                        aws --version
                        rm -rf AWSCLIV2.pkg aws
                        """
                ),
                StepModel(
                    title="Configure Aws credentials and environment",
                    description="",
                    runner=StepRunnerType.SHELL,
                    content="""
                    aws configure
                    """
                )
            ]),
    ])
)

io.YamlIO.to_file('./cocmd_cli_app/resources/demo/awscli/scripts/setup.yaml', data_awscli)




data_k8s = ScriptModel(
    name="setup",
    title="Setup Kubernetes",
    description="setup Kubernetes for desktop",
    spec=SpecModel(
        globals=[
            StepGlobalModel(
                    title="Instructions",
                    description="",
                    runner=StepRunnerType.MARKDOWN,
                    content="""
# Kubernetes is awesome
## cheers
### all good
                    """,
                    id="kube_instructions_md"
                )
        ],
        variations=[
        StepsModel(
            env=OS.LINUX,
            label="debian",
            steps=[
                StepRefModel(ref="kube_instructions_md"),
                StepModel(
                    title="Install kubectl",
                    description="This will install kubectl on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""
curl -LO \"https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl\"
curl -LO \"https://dl.k8s.io/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl.sha256\"
echo \"$(cat kubectl.sha256)  kubectl\" | sha256sum --check
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
kubectl version --client
rm -rf kubectl kubectl.sha256
"""
                ),
                StepModel(
                    title="Install kubectx",
                    description="",
                    runner=StepRunnerType.SHELL,
                    content="""
                    sudo git clone https://github.com/ahmetb/kubectx /usr/local/kubectx
                    sudo ln -s /usr/local/kubectx/kubectx /usr/local/bin/kubectx
                    sudo ln -s /usr/local/kubectx/kubens /usr/local/bin/kubens
                    """
                ),
                StepModel(
                    title="Install Minikube",
                    description="",
                    runner=StepRunnerType.SHELL,
                    content="""
                    
                    sudo apt-get install -y conntrack

                    # Download minikube
                    wget https://github.com/kubernetes/minikube/releases/download/v0.24.0/minikube_0.24-0.deb

                    # Install minikube
                    sudo dpkg -i minikube_0.24-0.deb

                    rm -rf minikube_0.24-0.deb

                    # Start minikube vm-driver=none option

                    sudo minikube start --vm-driver=none

                    # Change permission
                    sudo chown -R $USER $HOME/.kube
                    sudo chgrp -R $USER $HOME/.kube
                    sudo chown -R $USER $HOME/.minikube
                    sudo chgrp -R $USER $HOME/.minikube

                    # Make sure that minikube is running
                    kubectl get pods --all-namespaces
                    """
                )
            ]),

        StepsModel(
            env=OS.OSX,
            label=">10.x.x",
            depends=[
                "installed:brew",
            ],
            steps=[
                StepModel(
                    title="Install kubectl",
                    description="This will install kubectl on your machine",
                    runner=StepRunnerType.SHELL,
                    content="""curl -LO \"https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl\"
curl -LO \"https://dl.k8s.io/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl.sha256\"
echo \"$(cat kubectl.sha256)  kubectl\" | sha256sum --check
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
kubectl version --client
rm -rf kubectl kubectl.sha256
"""
                ),
                StepModel(
                    title="Install kubectx",
                    description="",
                    runner=StepRunnerType.SHELL,
                    content="""brew install kubectx"""
                ),
                StepModel(
                    title="Install Minikube",
                    description="",
                    runner=StepRunnerType.SHELL,
                    content="""brew install minikube"""
                )
            ]),
    ])
)

io.YamlIO.to_file('./cocmd_cli_app/resources/demo/k8s/scripts/setup.yaml', data_k8s)

