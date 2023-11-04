use reqwest::blocking;
use reqwest::header::USER_AGENT;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};
use strum::EnumIter;

enum ScriptType {
    Script,
    Alias,
}

#[derive(Debug)]
pub struct Script {
    pub raw: String,
}

impl Script {
    fn new<S: Into<String> + Clone>(script_type: ScriptType, path: S) -> Self {
        let raw_path: String = match script_type {
            ScriptType::Script => format!(
                "https://raw.githubusercontent.com/ShawnHeyli/proxbox/main/scripts/{}",
                path.clone().into()
            ),
            ScriptType::Alias => format!(
                "https://raw.githubusercontent.com/ShawnHeyli/proxbox/main/aliases/{}",
                path.clone().into()
            ),
        };

        let raw = blocking::Client::new()
            .get(raw_path)
            .header(USER_AGENT, "proxbox")
            .send()
            .expect("Unable to get raw file")
            .text()
            .expect("Unable to read raw file");

        Script { raw }
    }
}

#[derive(Debug, EnumIter)]
pub enum CompositeScript {
    Starship,
    AutoLogin,
    IDK,
}

impl CompositeScript {
    pub fn run(&self) -> Result<(), String> {
        match self {
            Self::Starship => starship(),
            Self::AutoLogin => auto_login(),
            _ => Err("Unknown script".to_string()),
        }
    }
}

impl Display for CompositeScript {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Starship => write!(f, "Starship | Script 1"),
            Self::AutoLogin => write!(f, "AutoLogin | Script 2"),
            _ => write!(f, "Unknown"),
        }
    }
}

fn starship() -> Result<(), String> {
    let starship = Script::new(ScriptType::Script, "starship.sh");
    let starship_prompt = Script::new(ScriptType::Alias, "starship_prompt.sh");
    let starship_sh = Script::new(ScriptType::Alias, "starship.sh");
    println!("{:?}", starship);
    Ok(())
}

fn auto_login() -> Result<(), String> {
    let auto_login = Script::new(ScriptType::Script, "auto_login.sh");
    let auto_login_sh = Script::new(ScriptType::Alias, "auto_login.sh");
    println!("{:?}", auto_login);
    Ok(())
}
