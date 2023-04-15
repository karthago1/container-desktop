# container-desktop
A tool for managing containers and images, volumes mounted into those containers.

The tool will be targeted for embedded systems devices. It will be lightweight so it can be installed everywhere. thanks to rust and the iced library WASM will be also supported

It is planned to support **docker** and **systemd-nspawn** container runtimes

## Features
- [x] Docker
- [x] systemd-nspawn
- [x] clone container (only nspawn)
- [x] list/start/stop containers
- [ ] WASM32 frontend
- [ ] backend webserver
- [ ] create / remove container and images

## Build

To build a desktop application run 
```
cargo build
cargo run
```
To build a WASM32 application run 
```
cd desktop
trunk build
trunk serve
```
