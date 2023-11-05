use reqwest::blocking;
use reqwest::header::USER_AGENT;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::fs::read_to_string;
use strum::EnumIter;

use crate::container::Container;
use crate::impls;

pub enum ScriptType {
    Script,
    Alias,
}

pub enum Location {
    Local,
    Remote,
}

#[derive(Debug)]
pub struct Script {
    pub raw: String,
    args: Option<Vec<String>>,
}

impl Script {
    pub fn new<S: Into<String> + Clone>(
        script_type: ScriptType,
        location: Location,
        path: S,
        args: Option<Vec<String>>,
    ) -> Self {
        let raw_path: String = match script_type {
            ScriptType::Script => match location {
                Location::Local => format!("scripts/{}", path.into()),
                Location::Remote => format!(
                    "https://raw.githubusercontent.com/proxbox-dev/proxbox/main/scripts/{}",
                    path.into()
                ),
            },
            ScriptType::Alias => match location {
                Location::Local => format!("aliases/{}", path.into()),
                Location::Remote => format!(
                    "https://raw.githubusercontent.com/proxbox-dev/proxbox/main/aliases/{}",
                    path.into()
                ),
            },
        };

        let raw = match location {
            Location::Local => read_to_string(raw_path).expect("Unable to read script"),
            Location::Remote => {
                let client = blocking::Client::new();
                let res = client
                    .get(&raw_path)
                    .header(USER_AGENT, "proxbox")
                    .send()
                    .unwrap()
                    .text()
                    .unwrap();
                res
            }
        };

        Script { raw, args }
    }

    pub fn run(&self, container: &Container) -> Result<(), String> {
        let raw = self.raw.clone();

        // Emulate bash behavior
        // Replace $1 with first arg
        // Replace $2 with second arg, etc... until 9 args
        let script = match &self.args {
            Some(args) => {
                let mut script = raw.clone();
                for (i, arg) in args.iter().enumerate() {
                    script = script.replace(&format!("${}", i + 1), arg);
                }
                script
            }
            None => raw,
        };

        container.run_string_in_lxc(script)
    }
}

#[derive(Debug, EnumIter)]
pub enum CompositeScript {
    Starship,
    AutoLogin,
    AlternativeShell,
}

impl CompositeScript {
    pub fn run(&self, container: &Container) -> Result<(), String> {
        match self {
            Self::Starship => impls::starship(container),
            Self::AutoLogin => impls::auto_login(container),
            Self::AlternativeShell => impls::choose_shell(container),
        }
    }
}

impl Display for CompositeScript {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Starship => write!(f, "Starship"),
            Self::AutoLogin => write!(f, "AutoLogin"),
            Self::AlternativeShell => write!(f, "Alternative shell | zsh/fish/nushell"),
        }
    }
}
