use crate::container::*;
use std::env;

mod container;
mod impls;
mod scripts;
mod utils;

fn main() {
    test().expect("Unable to test");
}

fn test() -> Result<(), Box<dyn std::error::Error>> {
    let container = Container::test();
    container.create()?;
    container.start()?;

    container.run_string_in_lxc("echo 'LC_ALL=en_US.UTF-8' >> /etc/environment")?;
    container.run_string_in_lxc("echo 'en_US.UTF-8 UTF-8' >> /etc/locale.gen")?;
    container.run_string_in_lxc("echo 'LANG=en_US.UTF-8' > /etc/locale.conf")?;
    match container.run_string_in_lxc("locale-gen en_US.UTF-8") {
        Ok(_) => {
            container.run_string_in_lxc("apt update")?;
            container.run_string_in_lxc("apt upgrade -y")?;
        }
        Err(_) => {
            container.run_string_in_lxc("apt clean")?;
            container.run_string_in_lxc("apt update")?;
            container.run_string_in_lxc("apt install -y locales")?;
            container.run_string_in_lxc("locale-gen en_US.UTF-8")?;
        }
    }

    container.run_string_in_lxc("apt update")?;
    container.run_string_in_lxc("apt upgrade -y")?;

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

    let scripts = container.select_scripts()?;
    container.run_composite_script_in_lxc(scripts)?;

    let scripts = container.select_scripts()?;
    container.run_composite_script_in_lxc(scripts)?;
    Ok(())
}
