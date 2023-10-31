use std::path::{Path, PathBuf};

use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input, Password};

pub fn select_file_from_dir<P: AsRef<Path>, S: Into<String>>(dir: P, msg: S) -> PathBuf {
    // get all files in ../scripts
    let scripts: Vec<String> = std::fs::read_dir(&dir)
        .expect("Unable to read directory /scripts")
        .map(|entry| {
            let path: PathBuf = entry.unwrap().path();
            path.file_name().unwrap().to_str().unwrap().to_string()
        })
        .collect();

    // Ask user to select a file
    let choice: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .default(0)
        .items(&scripts)
        .interact()
        .unwrap();

    // return the selected file
    Path::new(dir.as_ref()).join(&scripts[choice])
}

pub fn read_select_file_from_dir<P: AsRef<Path>, S: Into<String>>(dir: P, msg: S) -> String {
    // read the selected file
    std::fs::read_to_string(select_file_from_dir(dir, msg)).expect("Unable to read file")
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
