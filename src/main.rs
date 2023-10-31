use std::process::exit;

use cmd_lib::run_cmd;
use proxbox::{get_int, get_secret, get_string};
fn main() {
    mvp();
}

fn mvp() {
    let id = get_int("Enter ID:");
    if id < 100 || id > 99999 {
        eprintln!("Invalid ID: {}", id);
        exit(1);
    }
    let template = get_string("Enter template:");
    let hostname = get_string("Enter hostname:");
    let password = get_secret("Enter password:");
    let cores = get_int("Enter cores:");
    let memory = get_int("Enter memory:");
    let swap = get_int("Enter swap:");
    let disk = get_string("Enter disk:");
    let storage = get_string("Enter storage:");
    let net = get_string("Enter net:");

    run_cmd!(pct create $id $template -hostname $hostname -password $password -cores $cores -memory $memory -swap $swap -net0 $net -storage $storage -rootfs $disk).expect("Unable to create LXC");
    run_cmd!(pct start $id).expect("Unable to start LXC");
}
