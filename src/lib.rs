use cmd_lib::run_cmd;
use dialoguer::{Input, MultiSelect, Password};
use reqwest::blocking;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::fmt::Display;

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

pub fn run_selected_scripts() -> () {
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

        run_cmd!(pct exec 105 -- /bin/bash -c "$script").expect("Unable to run script");
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
