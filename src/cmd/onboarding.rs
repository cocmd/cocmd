use anyhow::Result;
use chatgpt::{prelude::*, types::CompletionResponse};
use dotenv::dotenv;
use log::error;

use crate::core::packages_manager::PackagesManager;

pub async fn run_onboarding(_packages_manager: &mut PackagesManager) -> Result<()> {
    dotenv().ok();
    let key = std::env::var("OPENAI_API_KEY")?;

    let client = ChatGPT::new(key)?;

    // Sending a message and getting the completion
    let response = client
        .send_message(
            "
        available playbooks names and when to use:
        - node.setup - the project has node requirements
        - python.setup - the project has python requirements
        - docker.setup - the project has docker requirements
        - kubernetes.setup - the project has kubernetes requirements
        - helm.setup - the project has helm requirements
        - terraform.setup - the project has terraform requirements
        - ansible.setup - the project has ansible requirements
        - golang.setup - the project has golang requirements
        - java.setup - the project has java requirements
        - ruby.setup - the project has ruby requirements
        - php.setup - the project has php requirements
        - rust.setup - the project has rust requirements
        - c.setup - the project has c requirements
        - c++.setup - the project has c++ requirements
        - swift.setup - the project has swift requirements
        - kotlin.setup - the project has kotlin requirements
        - scala.setup - the project has scala requirements
        - clojure.setup - the project has clojure requirements
        - elixir.setup - the project has elixir requirements
        - erlang.setup - the project has erlang requirements
        - haskell.setup - the project has haskell requirements
        - lua.setup - the project has lua requirements
        
        
        Args:
        playbook : name of the package and playbook to execute. 
        file_hint : the file in the project made for reason this is the playbook needed.
        \"\"\"
        <docstring_end>
        
        
        User Query: call the call_playbook function several times with the right playbook name according to a 
        dev project tree output i have:
        
        ├── CONTRIBUTING.md
        ├── LICENSE
        ├── README.md
        ├── check_ports.sh
        ├── container-storage
        ├── docker-compose.test-github.yml
        ├── docker-compose.yml
        ├── docker-grafana
        ├── docker-influxdb
        ├── docker-mosquitto
        ├── docker-python
        ├── docker-python-pypy
        ├── docker-redis
        ├── import.sh
        ├── krakend.json
        ├── kubernetes
        ├── python
        ├── requirements.txt
        └── wait_until_up.sh    
        

        call the function for every playbook you see needs to be a part of the project.
        ",
        )
        .await;

    if let Err(err) = response {
        error!("Error: {}", err);
        return Err(err.into());
    }

    let response: CompletionResponse = response.unwrap();

    println!("Response: {}", response.message().content);

    Ok(())
}
