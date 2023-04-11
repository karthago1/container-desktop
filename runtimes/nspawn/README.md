the systemd.rs and system_machine.rs where automatically generated with the following commands:
```
dbus-codegen-rust -s -g -m None -d org.freedesktop.systemd1 -p /org/freedesktop/systemd1 > systemd.rs
dbus-codegen-rust -s -g -m None -d org.freedesktop.machine1 -p /org/freedesktop/systemd1 > systemd_machine.rs
```
