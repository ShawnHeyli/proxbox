use crate::scripts::*;
use cmd_lib::{run_cmd, run_fun};
use dialoguer::{Input, MultiSelect, Password};

pub fn get_int<S: Into<String>>(msg: S) -> i32 {
    Input::<i32>::new().with_prompt(msg).interact().unwrap()
}

pub fn get_string<S: Into<String>>(msg: S) -> String {
    Input::<String>::new().with_prompt(msg).interact().unwrap()
}

pub fn get_secret<S: Into<String>>(msg: S) -> String {
    Password::new().with_prompt(msg).interact().unwrap()
}
