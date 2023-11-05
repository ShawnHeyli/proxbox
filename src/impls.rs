use crate::{
    container::Container,
    scripts::{Location, Script, ScriptType},
};
use dialoguer::Select;

pub fn starship(container: &Container) -> Result<(), String> {
    let presets: Vec<String> = vec!["no-nerd-font".into(), "plain-text-symbols".into()];

    let prompt_preset = Select::new()
        .with_prompt("Select a preset")
        .default(0)
        .items(&presets)
        .interact()
        .unwrap();

    let script = Script::new(
        ScriptType::Script,
        Location::Local,
        "starship.sh",
        Some(vec![presets[prompt_preset].clone()]),
    );

    container.run_script_in_lxc(&script)?;
    println!("Starship prompt installed & enabled");
    Ok(())
}

pub fn auto_login(container: &Container) -> Result<(), String> {
    let script = Script::new(ScriptType::Script, Location::Remote, "auto-login.sh", None);
    container.run_script_in_lxc(&script)?;
    println!("Auto login enabled");
    Ok(())
}

pub fn choose_shell(container: &Container) -> Result<(), String> {
    let shells: Vec<String> = vec!["zsh".into(), "fish".into(), "nushell".into()];

    let prompt_shell = Select::new()
        .with_prompt("Install an alternative shell")
        .default(0)
        .items(&shells)
        .interact()
        .unwrap();

    let script = Script::new(
        ScriptType::Script,
        Location::Local,
        format!("shells/install_{}.sh", shells[prompt_shell].to_lowercase()).as_str(),
        None,
    );

    container.run_script_in_lxc(&script)?;
    println!("Shell changed");
    Ok(())
}
