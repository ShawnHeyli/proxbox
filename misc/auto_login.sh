override="
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin root --noclear --keep-baud tty%I 115200,38400,9600 $TERM
"

mkdir -p /etc/systemd/system/container-getty@1.service.d/
echo $override >> /etc/systemd/system/container-getty@1.service.d/override.conf

systemctl daemon-reload
systemctl restart container-getty@1.service