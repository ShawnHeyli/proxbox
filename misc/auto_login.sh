override_getty="/etc/systemd/system/container-getty@1.service.d/override.conf"
container_id=100

pct exec $container_id -- bash -c "mkdir -p $(dirname $override_getty)"

override="
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin root --noclear --keep-baud tty%I 115200,38400,9600 \$TERM
"

script="
systemctl daemon-reload
systemctl restart
"

pct exec $container_id -- bash -c "echo '$override' > $override_getty"
pct exec $container_id -- bash -c "systemctl daemon-reload"
pct exec $container_id -- bash -c "echo '$override' > $override_getty"


let override_getty = "/etc/systemd/system/container-getty@1.service.d/override.conf";
let dir_override_getty = "/etc/systemd/system/container-getty@1.service.d/override.conf";
run_cmd!(pct exec 100 -- bash -c "mkdir -p $(dirname $override_getty)").unwrap();
// Create a file
let idk = "[Service]\nExecStart=\nExecStart=-/sbin/agetty --autologin root --noclear --keep-baud tty%I 115200,38400,9600 \\$TERM";
fs::write("./auto_login", idk);
run_cmd!(cp auto_login $override_getty).unwrap();
run_cmd!(systemctl daemon-reload);
run_cmd!(systemctl restart $override_getty);