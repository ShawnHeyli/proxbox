use cmd_lib::{run_cmd, run_fun};
use dialoguer::{Input, MultiSelect, Password};
use reqwest::blocking;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Script {
    name: String,
    download_url: String,
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.download_url)
    }
}

pub fn run_selected_scripts(id: i32) -> () {
    let scripts: Vec<Script> = blocking::Client::new()
        .get("https://api.github.com/repos/ShawnHeyli/proxbox/contents/scripts")
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0",
        )
        .send()
        .expect("Unable to download scripts")
        .json::<Vec<Script>>()
        .expect("Unable to read scripts");

    let choices = MultiSelect::new()
        .items(&scripts)
        .interact()
        .expect("Unable to select script");

    for choice in choices {
        let script = blocking::get(&scripts[choice].download_url)
            .expect("Unable to download script")
            .text()
            .expect("Unable to download script");

        run_script_in_lxc(id, script);
    }
}

pub fn run_script_in_lxc<S: ToString>(id: i32, script: S) -> () {
    // run the command until it works, because the LXC might not be ready yet
    // lxc-attach and pct exec are the same command, but I'm scared to change it
    loop {
        match run_cmd!(lxc-attach -n $id -- ps aux &> /dev/null) {
            Ok(_) => {
                run_cmd!(pct exec $id -- bash -c "$script").expect("Unable to run script");
                break;
            }
            Err(_) => sleep(Duration::from_secs(1)),
        }
    }
}

pub fn get_int<S: Into<String>>(msg: S) -> i32 {
    Input::<i32>::new().with_prompt(msg).interact().unwrap()
}

pub fn get_string<S: Into<String>>(msg: S) -> String {
    Input::<String>::new().with_prompt(msg).interact().unwrap()
}

pub fn get_secret<S: Into<String>>(msg: S) -> String {
    Password::new().with_prompt(msg).interact().unwrap()
}

pub fn get_id<S: Into<String>>(msg: S) -> Result<i32, String> {
    let id = get_int(msg);
    if id < 100 || id > 99999 {
        Err("ID must be between 100 and 99999".to_string())
    } else {
        Ok(id)
    }
}

pub fn get_password<S: Into<String>>(msg: S) -> Result<String, String> {
    let password = get_secret(msg);
    if password.len() < 8 {
        Err("Password must be at least 8 characters".to_string())
    } else {
        Ok(password)
    }
}

pub fn get_disk<M: Into<String>, S: Display>(msg: M, storage: S) -> String {
    let disk = get_int(msg);
    // TODO handle template storage location (hardcoded to mergerfs here)
    format!("{storage}:{disk}")
}

pub fn get_debian_template() -> String {
    let template = run_fun!(pveam available -section system | grep debian-12 | cut -d " " -f 11)
        .expect("Unable to find debian template");

    // Download template
    // TODO handle template storage location (hardcoded to mergerfs here)
    run_cmd!(pveam download mergerfs $template).expect("Unable to download debian template");
    return format!("mergerfs:vztmpl/{template}");
}
