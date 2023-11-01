use std::thread::sleep;
use std::{fmt::Display, process::exit, time::Duration};

use cmd_lib::{run_cmd, run_fun};
use proxbox::{get_int, get_secret, get_string, run_selected_scripts};

fn main() {
    debian().expect("Could not create debian LXC");
}

fn get_id<S: Into<String>>(msg: S) -> i32 {
    let id = get_int(msg);
    if id < 100 || id > 99999 {
        eprintln!("Invalid ID: {}", id);
        exit(1);
    } else {
        return id;
    }
}

fn get_password<S: Into<String>>(msg: S) -> String {
    let password = get_secret(msg);
    if password.len() < 8 {
        eprintln!("Password must be at least 8 characters");
        exit(1);
    } else {
        return password;
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

fn debian() -> Result<(), Box<dyn std::error::Error>> {
    let id = get_id("Enter ID:");
    let template = get_debian_template();
    let hostname = get_string("Enter hostname:");
    let password = get_password("Enter password:");
    let cores = get_int("Enter cores:");
    let memory = get_int("Enter memory:");
    let swap = get_int("Enter swap:");
    let storage = get_string("Enter storage:");
    let disk = get_disk("Enter disk size (Gib):", storage.clone());
    let net0 = get_string("Enter net:");

    run_cmd!(pct create $id $template -hostname $hostname -password $password -cores $cores -memory $memory -swap $swap -net0 $net0 -storage $storage -rootfs $disk).expect("Unable to create LXC");
    run_cmd!(pct start $id).expect("Unable to start LXC");

    println!("LXC created. Waiting for LXC to start...");

    println!("LXC started. Running auto_login.sh...");

    // run the command until it works, because the LXC might not be ready yet
    loop {
        match run_cmd!(lxc-attach -n $id -- ps aux &> /dev/null) {
            Ok(_) => {
                run_selected_scripts();
                break;
            }
            Err(_) => sleep(Duration::from_secs(1)),
        }
    }

    Ok(())
}
