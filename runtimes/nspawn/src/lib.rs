use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Result;
use async_trait::async_trait;
use container_core::{
    container::{Container, ContainerProvider},
    image::{Image, ImageProvider},
    CorePlugin,
};
use dbus::blocking::{Connection, Proxy};

mod systemd;
mod systemd_machine;

static DBUS_DEST: &str = "org.freedesktop.machine1";
static DBUS_IFACE: &str = "/org/freedesktop/machine1";

static DBUS_SYSTEMD_DEST: &str = "org.freedesktop.systemd1";
static DBUS_SYSTEMD_IFACE: &str = "/org/freedesktop/systemd1";

struct Client {
    con: Arc<Mutex<Connection>>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            con: Arc::new(Mutex::new(Connection::new_system().unwrap())),
        }
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish()
    }
}

impl CorePlugin for Client {
    fn is_clone_supported(&self) -> bool {
        true
    }
}

#[async_trait]
impl ImageProvider for Client {
    async fn list_images(&self) -> Result<Vec<Image>> {
        /*let guard = self.con.lock();
        match guard {
            Ok(guard) => {
                let proxy = guard.with_proxy(DBUS_DEST, DBUS_IFACE, Duration::from_millis(5000));
                use systemd_machine::OrgFreedesktopMachine1Manager;

                let imgs = proxy.list_images()?;

                Ok(imgs
                    .into_iter()
                    .filter(|(n, _, _, _, _, _, _)| !n.starts_with('.') || n == ".host")
                    .map(|(n, _t, _ro, _created, _mtime, size, _p)| {
                        Image::new(n.clone(), n, size as usize)
                    })
                    .collect())
            }
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }*/
        Ok(vec![])
    }
}

fn read_image(proxy: &Proxy<&Connection>, name: &str) -> String {
    use systemd_machine::OrgFreedesktopMachine1Manager;
    let res = proxy.get_image_osrelease(&name);
    if let Ok(dict) = res {
        let pretty = dict.get("PRETTY_NAME");
        if let Some(pretty) = pretty {
            pretty.clone()
        } else {
            let os = dict.get("ID");
            let mut image = if let Some(os) = os {
                os.clone()
            } else {
                "".to_string()
            };

            let ver = dict.get("VERSION");
            let ver_id = dict.get("VERSION_ID");
            if let Some(ver) = ver {
                image = image + " " + ver;
            } else if let Some(ver_id) = ver_id {
                image = image + " " + ver_id
            }
            image
        }
    } else {
        "".to_string()
    }
}

fn generate_service_name(name: &str) -> String {
    "systemd-nspawn@".to_string() + name + ".service"
}

#[async_trait]
impl ContainerProvider for Client {
    async fn list_containers(&self) -> Result<Vec<Container>> {
        let guard = self.con.lock();
        match guard {
            Ok(guard) => {
                let proxy = guard.with_proxy(DBUS_DEST, DBUS_IFACE, Duration::from_millis(5000));
                use systemd_machine::OrgFreedesktopMachine1Manager;

                let imgs = proxy.list_images()?;

                let machines = proxy.list_machines()?;

                let containers: Vec<Container> = imgs
                    .into_iter()
                    .filter(|(n, _, _, _, _, _, _)| !n.starts_with('.') || n == ".host")
                    .map(|(n, t, _ro, _created, _mtime, _size, _p)| {
                        let found = machines.iter().find(|&e| e.0 == n);
                        let img = read_image(&proxy, &n);
                        Container::new(n, "".to_string(), img, found.is_some(), t)
                    })
                    .collect();

                Ok(containers)
            }
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }

    async fn start_container(&self, id: String) -> Result<()> {
        println!("start container {id}");
        let guard = self.con.lock();
        match guard {
            Ok(guard) => {
                let proxy = guard.with_proxy(
                    DBUS_SYSTEMD_DEST,
                    DBUS_SYSTEMD_IFACE,
                    Duration::from_millis(5000),
                );
                use systemd::OrgFreedesktopSystemd1Manager;
                let service_name = generate_service_name(&id);
                let _ = proxy.start_unit(&service_name, "fail")?;
                Ok(())
            }
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }

    async fn stop_container(&self, id: String) -> Result<()> {
        println!("stop container {id}");
        let guard = self.con.lock();
        match guard {
            Ok(guard) => {
                let proxy = guard.with_proxy(
                    DBUS_SYSTEMD_DEST,
                    DBUS_SYSTEMD_IFACE,
                    Duration::from_millis(5000),
                );
                //use systemd_machine::OrgFreedesktopMachine1Manager;
                //proxy.kill_machine(&id, "leader", SIGRTMIN + 4)?;

                use systemd::OrgFreedesktopSystemd1Manager;
                let service_name = generate_service_name(&id);
                let _ = proxy.stop_unit(&service_name, "fail")?;
                Ok(())
            }
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }

    async fn clone_container(&self, id: String, new_name: String) -> Result<()> {
        let guard = self.con.lock();
        match guard {
            Ok(guard) => {
                let proxy = guard.with_proxy(DBUS_DEST, DBUS_IFACE, Duration::from_millis(5000));
                use systemd_machine::OrgFreedesktopMachine1Manager;
                proxy.clone_image(&id, &new_name, false)?;
                Ok(())
            }
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }
}

#[no_mangle]
pub fn initialize() -> Box<dyn CorePlugin> {
    Box::<Client>::default()
}
