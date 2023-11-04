use cmd_lib::{run_cmd, run_fun};
use dialoguer::MultiSelect;
use std::fmt::Display;
use strum::IntoEnumIterator;

use crate::scripts::{CompositeScript, Script};
use crate::utils::{get_int, get_secret, get_string};

pub struct Container {
    id: i32,
    template: String,
    hostname: String,
    password: String,
    cores: i32,
    memory: i32,
    swap: i32,
    storage: String,
    disk: String,
    net0: String,
}

impl Container {
    pub fn new() -> Container {
        let id = get_id("Enter ID:").expect("Unable to get ID");
        let template = get_debian_template();
        let hostname = get_string("Enter hostname:");
        let password = get_password("Enter password:").expect("Unable to get password");
        let cores = get_int("Enter cores:");
        let memory = get_int("Enter memory:");
        let swap = get_int("Enter swap:");
        let storage = get_string("Enter storage:");
        let disk = get_disk("Enter disk size (Gib):", storage.clone());
        let net0 = get_string("Enter net:");
        Container {
            id,
            template,
            hostname,
            password,
            cores,
            memory,
            swap,
            storage,
            disk,
            net0,
        }
    }

    // FOR DEBUG PURPOSES ONLY
    pub fn test() -> Container {
        Container {
            id: 105,
            template: "mergerfs:vztmpl/debian-12-standard_12.2-1_amd64.tar.zst".into(),
            hostname: "mvp".into(),
            password: "password".into(),
            cores: 1,
            memory: 512,
            swap: 512,
            storage: "mergerfs".into(),
            disk: "mergerfs:4".into(),
            net0: "name=eth0,bridge=vmbr0,ip=dhcp".into(),
        }
    }

    pub fn create(&self) -> Result<(), String> {
        let id = self.id;
        let template = &self.template;
        let hostname = &self.hostname;
        let password = &self.password;
        let cores = self.cores;
        let memory = self.memory;
        let swap = self.swap;
        let storage = &self.storage;
        let disk = &self.disk;
        let net0 = &self.net0;
        match run_cmd!(pct create $id $template -hostname $hostname -password $password -cores $cores -memory $memory -swap $swap -net0 $net0 -storage $storage -rootfs $disk)
        {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to create LXC".into()),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let id = self.id;
        let res = match run_cmd!(pct start $id) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to start the {} LXC", self.id)),
        };
        let _ = self.wait_for_startup();
        return res;
    }

    fn wait_for_startup(&self) -> Result<(), String> {
        let id = self.id;
        match run_cmd!(lxc-attach -n $id -- bash -c "echo 'LXC is ready'") {
            Ok(_) => Ok(()),
            Err(_) => self.wait_for_startup(),
        }
    }

    pub fn run_string_in_lxc<S: ToString>(&self, script: S) -> Result<(), String> {
        let id = self.id;
        match run_cmd!(lxc-attach -n $id -- bash -c "$script") {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to run script in the {} LXC", self.id)),
        }
    }

    pub fn run_script_in_lxc(&self, script: &Script) -> Result<(), String> {
        let id = self.id;
        let script: String = script.raw.clone();
        match run_cmd!(lxc-attach -n $id -- bash -c "curl -s https://raw.githubusercontent.com/brandon1024/proxmox-scripts/main/scripts/$script | bash")
        {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to run script in the {} LXC", self.id)),
        }
    }

    pub fn run_composite_script_in_lxc(&self, script: Vec<usize>) -> Result<(), String> {
        for script in script {
            CompositeScript::iter().nth(script).unwrap().run()?;
        }
        Ok(())
    }

    pub fn select_scripts(&self) -> Result<Vec<usize>, String> {
        let scripts: Vec<usize> = MultiSelect::new()
            .with_prompt("Which script do you want to run?")
            .items(&CompositeScript::iter().collect::<Vec<_>>())
            .interact()
            .unwrap();

        Ok(scripts)
    }
}

fn get_id<S: Into<String>>(msg: S) -> Result<i32, String> {
    let id = get_int(msg);
    if id < 100 || id > 99999 {
        Err("ID must be between 100 and 99999".to_string())
    } else {
        Ok(id)
    }
}

fn get_password<S: Into<String>>(msg: S) -> Result<String, String> {
    let password = get_secret(msg);
    if password.len() < 8 {
        Err("Password must be at least 8 characters".to_string())
    } else {
        Ok(password)
    }
}

fn get_disk<M: Into<String>, S: Display>(msg: M, storage: S) -> String {
    let disk = get_int(msg);
    // TODO handle template storage location (hardcoded to mergerfs here)
    format!("{storage}:{disk}")
}

fn get_debian_template() -> String {
    let template = run_fun!(pveam available -section system | grep debian-12 | cut -d " " -f 11)
        .expect("Unable to find debian template");

    // Download template
    // TODO handle template storage location (hardcoded to mergerfs here)
    run_cmd!(pveam download mergerfs $template).expect("Unable to download debian template");
    return format!("mergerfs:vztmpl/{template}");
}
