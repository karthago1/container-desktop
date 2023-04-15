# container-desktop
A tool for managing containers and images, volumes mounted into those containers.

The tool will be targeted for embedded systems devices. It will be lightweight so it can be installed everywhere. thanks to rust and the iced library WASM will be also supported

It is planned to support **docker** and **systemd-nspawn** container runtimes

## Features
- [x] Docker
- [x] systemd-nspawn
- [x] clone container (only nspawn)
- [x] list/start/stop containers
- [ ] create / remove container and images
