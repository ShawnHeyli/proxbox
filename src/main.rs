use crate::container::*;

mod container;
mod scripts;
mod utils;

fn main() {
    test().expect("Unable to test");
}

fn test() -> Result<(), Box<dyn std::error::Error>> {
    let container = Container::test();
    //container.create()?;
    //container.start()?;

    let scripts = container.select_scripts()?;
    container.run_composite_script_in_lxc(scripts)?;

    Ok(())
}

fn debian() -> Result<(), Box<dyn std::error::Error>> {
    let container = Container::new();
    container.create()?;
    container.start()?;

    container.run_string_in_lxc("apt update")?;
    container.run_string_in_lxc("apt upgrade -y")?;

    Ok(())
}
