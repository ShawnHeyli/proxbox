use cmd_lib::run_cmd;
use proxbox::*;

fn main() {
    debian().expect("Could not create debian LXC");
}

fn debian() -> Result<(), Box<dyn std::error::Error>> {
    let id = get_id("Enter ID:").expect("Unable to get ID");
    let template = get_debian_template();
    let hostname = get_string("Enter hostname:");
    let password = get_password("Enter password:").expect("Unable to get password");
    let cores = get_int("Enter cores:");
    let memory = get_int("Enter memory:");
    let swap = get_int("Enter swap:");
    let storage = get_string("Enter storage:");
    let disk = get_disk("Enter disk size (Gib):", storage.clone());
    let net0 = get_string("Enter net:");

    run_cmd!(pct create $id $template -hostname $hostname -password $password -cores $cores -memory $memory -swap $swap -net0 $net0 -storage $storage -rootfs $disk).expect("Unable to create LXC");
    run_cmd!(pct start $id).expect("Unable to start LXC");

    println!("LXC created. Waiting for LXC to start...");

    println!("LXC started. Running scripts...");

    run_script_in_lxc(id, "apt update");
    run_script_in_lxc(id, "apt upgrade -y");
    run_selected_scripts(id);

    Ok(())
}
