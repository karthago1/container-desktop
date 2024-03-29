#![allow(clippy::all)]
// This code was autogenerated with `dbus-codegen-rust -s -g -m None -d org.freedesktop.machine1 -p /org/freedesktop/machine1`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusProperties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<R0>, )| Ok((r.0).0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, arg::Variant(value), ))
    }
}

pub trait OrgFreedesktopMachine1Manager {
    fn get_machine(&self, name: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn get_image(&self, name: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn get_machine_by_pid(&self, pid: u32) -> Result<dbus::Path<'static>, dbus::Error>;
    fn list_machines(&self) -> Result<Vec<(String, String, String, dbus::Path<'static>)>, dbus::Error>;
    fn list_images(&self) -> Result<Vec<(String, String, bool, u64, u64, u64, dbus::Path<'static>)>, dbus::Error>;
    fn create_machine(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str, scope_properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>) -> Result<dbus::Path<'static>, dbus::Error>;
    fn create_machine_with_network(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str, ifindices: Vec<i32>, scope_properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>) -> Result<dbus::Path<'static>, dbus::Error>;
    fn register_machine(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn register_machine_with_network(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str, ifindices: Vec<i32>) -> Result<dbus::Path<'static>, dbus::Error>;
    fn unregister_machine(&self, name: &str) -> Result<(), dbus::Error>;
    fn terminate_machine(&self, id: &str) -> Result<(), dbus::Error>;
    fn kill_machine(&self, name: &str, who: &str, signal: i32) -> Result<(), dbus::Error>;
    fn get_machine_addresses(&self, name: &str) -> Result<Vec<(i32, Vec<u8>)>, dbus::Error>;
    fn get_machine_osrelease(&self, name: &str) -> Result<::std::collections::HashMap<String, String>, dbus::Error>;
    fn open_machine_pty(&self, name: &str) -> Result<(arg::OwnedFd, String), dbus::Error>;
    fn open_machine_login(&self, name: &str) -> Result<(arg::OwnedFd, String), dbus::Error>;
    fn open_machine_shell(&self, name: &str, user: &str, path: &str, args: Vec<&str>, environment: Vec<&str>) -> Result<(arg::OwnedFd, String), dbus::Error>;
    fn bind_mount_machine(&self, name: &str, source: &str, destination: &str, read_only: bool, mkdir: bool) -> Result<(), dbus::Error>;
    fn copy_from_machine(&self, name: &str, source: &str, destination: &str) -> Result<(), dbus::Error>;
    fn copy_to_machine(&self, name: &str, source: &str, destination: &str) -> Result<(), dbus::Error>;
    fn copy_from_machine_with_flags(&self, name: &str, source: &str, destination: &str, flags: u64) -> Result<(), dbus::Error>;
    fn copy_to_machine_with_flags(&self, name: &str, source: &str, destination: &str, flags: u64) -> Result<(), dbus::Error>;
    fn open_machine_root_directory(&self, name: &str) -> Result<arg::OwnedFd, dbus::Error>;
    fn get_machine_uidshift(&self, name: &str) -> Result<u32, dbus::Error>;
    fn remove_image(&self, name: &str) -> Result<(), dbus::Error>;
    fn rename_image(&self, name: &str, new_name: &str) -> Result<(), dbus::Error>;
    fn clone_image(&self, name: &str, new_name: &str, read_only: bool) -> Result<(), dbus::Error>;
    fn mark_image_read_only(&self, name: &str, read_only: bool) -> Result<(), dbus::Error>;
    fn get_image_hostname(&self, name: &str) -> Result<String, dbus::Error>;
    fn get_image_machine_id(&self, name: &str) -> Result<Vec<u8>, dbus::Error>;
    fn get_image_machine_info(&self, name: &str) -> Result<::std::collections::HashMap<String, String>, dbus::Error>;
    fn get_image_osrelease(&self, name: &str) -> Result<::std::collections::HashMap<String, String>, dbus::Error>;
    fn set_pool_limit_(&self, size: u64) -> Result<(), dbus::Error>;
    fn set_image_limit(&self, name: &str, size: u64) -> Result<(), dbus::Error>;
    fn clean_pool(&self, mode: &str) -> Result<Vec<(String, u64)>, dbus::Error>;
    fn map_from_machine_user(&self, name: &str, uid_inner: u32) -> Result<u32, dbus::Error>;
    fn map_to_machine_user(&self, uid_outer: u32) -> Result<(String, dbus::Path<'static>, u32), dbus::Error>;
    fn map_from_machine_group(&self, name: &str, gid_inner: u32) -> Result<u32, dbus::Error>;
    fn map_to_machine_group(&self, gid_outer: u32) -> Result<(String, dbus::Path<'static>, u32), dbus::Error>;
    fn pool_path(&self) -> Result<String, dbus::Error>;
    fn pool_usage(&self) -> Result<u64, dbus::Error>;
    fn pool_limit(&self) -> Result<u64, dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopMachine1ManagerMachineNew {
    pub machine: String,
    pub path: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopMachine1ManagerMachineNew {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.machine, i);
        arg::RefArg::append(&self.path, i);
    }
}

impl arg::ReadAll for OrgFreedesktopMachine1ManagerMachineNew {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopMachine1ManagerMachineNew {
            machine: i.read()?,
            path: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopMachine1ManagerMachineNew {
    const NAME: &'static str = "MachineNew";
    const INTERFACE: &'static str = "org.freedesktop.machine1.Manager";
}

#[derive(Debug)]
pub struct OrgFreedesktopMachine1ManagerMachineRemoved {
    pub machine: String,
    pub path: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopMachine1ManagerMachineRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.machine, i);
        arg::RefArg::append(&self.path, i);
    }
}

impl arg::ReadAll for OrgFreedesktopMachine1ManagerMachineRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopMachine1ManagerMachineRemoved {
            machine: i.read()?,
            path: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopMachine1ManagerMachineRemoved {
    const NAME: &'static str = "MachineRemoved";
    const INTERFACE: &'static str = "org.freedesktop.machine1.Manager";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopMachine1Manager for blocking::Proxy<'a, C> {

    fn get_machine(&self, name: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetMachine", (name, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn get_image(&self, name: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetImage", (name, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn get_machine_by_pid(&self, pid: u32) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetMachineByPID", (pid, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn list_machines(&self) -> Result<Vec<(String, String, String, dbus::Path<'static>)>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "ListMachines", ())
            .and_then(|r: (Vec<(String, String, String, dbus::Path<'static>)>, )| Ok(r.0, ))
    }

    fn list_images(&self) -> Result<Vec<(String, String, bool, u64, u64, u64, dbus::Path<'static>)>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "ListImages", ())
            .and_then(|r: (Vec<(String, String, bool, u64, u64, u64, dbus::Path<'static>)>, )| Ok(r.0, ))
    }

    fn create_machine(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str, scope_properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CreateMachine", (name, id, service, class, leader, root_directory, scope_properties, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn create_machine_with_network(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str, ifindices: Vec<i32>, scope_properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CreateMachineWithNetwork", (name, id, service, class, leader, root_directory, ifindices, scope_properties, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn register_machine(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "RegisterMachine", (name, id, service, class, leader, root_directory, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn register_machine_with_network(&self, name: &str, id: Vec<u8>, service: &str, class: &str, leader: u32, root_directory: &str, ifindices: Vec<i32>) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "RegisterMachineWithNetwork", (name, id, service, class, leader, root_directory, ifindices, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn unregister_machine(&self, name: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "UnregisterMachine", (name, ))
    }

    fn terminate_machine(&self, id: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "TerminateMachine", (id, ))
    }

    fn kill_machine(&self, name: &str, who: &str, signal: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "KillMachine", (name, who, signal, ))
    }

    fn get_machine_addresses(&self, name: &str) -> Result<Vec<(i32, Vec<u8>)>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetMachineAddresses", (name, ))
            .and_then(|r: (Vec<(i32, Vec<u8>)>, )| Ok(r.0, ))
    }

    fn get_machine_osrelease(&self, name: &str) -> Result<::std::collections::HashMap<String, String>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetMachineOSRelease", (name, ))
            .and_then(|r: (::std::collections::HashMap<String, String>, )| Ok(r.0, ))
    }

    fn open_machine_pty(&self, name: &str) -> Result<(arg::OwnedFd, String), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "OpenMachinePTY", (name, ))
    }

    fn open_machine_login(&self, name: &str) -> Result<(arg::OwnedFd, String), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "OpenMachineLogin", (name, ))
    }

    fn open_machine_shell(&self, name: &str, user: &str, path: &str, args: Vec<&str>, environment: Vec<&str>) -> Result<(arg::OwnedFd, String), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "OpenMachineShell", (name, user, path, args, environment, ))
    }

    fn bind_mount_machine(&self, name: &str, source: &str, destination: &str, read_only: bool, mkdir: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "BindMountMachine", (name, source, destination, read_only, mkdir, ))
    }

    fn copy_from_machine(&self, name: &str, source: &str, destination: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CopyFromMachine", (name, source, destination, ))
    }

    fn copy_to_machine(&self, name: &str, source: &str, destination: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CopyToMachine", (name, source, destination, ))
    }

    fn copy_from_machine_with_flags(&self, name: &str, source: &str, destination: &str, flags: u64) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CopyFromMachineWithFlags", (name, source, destination, flags, ))
    }

    fn copy_to_machine_with_flags(&self, name: &str, source: &str, destination: &str, flags: u64) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CopyToMachineWithFlags", (name, source, destination, flags, ))
    }

    fn open_machine_root_directory(&self, name: &str) -> Result<arg::OwnedFd, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "OpenMachineRootDirectory", (name, ))
            .and_then(|r: (arg::OwnedFd, )| Ok(r.0, ))
    }

    fn get_machine_uidshift(&self, name: &str) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetMachineUIDShift", (name, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn remove_image(&self, name: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "RemoveImage", (name, ))
    }

    fn rename_image(&self, name: &str, new_name: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "RenameImage", (name, new_name, ))
    }

    fn clone_image(&self, name: &str, new_name: &str, read_only: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CloneImage", (name, new_name, read_only, ))
    }

    fn mark_image_read_only(&self, name: &str, read_only: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "MarkImageReadOnly", (name, read_only, ))
    }

    fn get_image_hostname(&self, name: &str) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetImageHostname", (name, ))
            .and_then(|r: (String, )| Ok(r.0, ))
    }

    fn get_image_machine_id(&self, name: &str) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetImageMachineID", (name, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_image_machine_info(&self, name: &str) -> Result<::std::collections::HashMap<String, String>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetImageMachineInfo", (name, ))
            .and_then(|r: (::std::collections::HashMap<String, String>, )| Ok(r.0, ))
    }

    fn get_image_osrelease(&self, name: &str) -> Result<::std::collections::HashMap<String, String>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "GetImageOSRelease", (name, ))
            .and_then(|r: (::std::collections::HashMap<String, String>, )| Ok(r.0, ))
    }

    fn set_pool_limit_(&self, size: u64) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "SetPoolLimit", (size, ))
    }

    fn set_image_limit(&self, name: &str, size: u64) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "SetImageLimit", (name, size, ))
    }

    fn clean_pool(&self, mode: &str) -> Result<Vec<(String, u64)>, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "CleanPool", (mode, ))
            .and_then(|r: (Vec<(String, u64)>, )| Ok(r.0, ))
    }

    fn map_from_machine_user(&self, name: &str, uid_inner: u32) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "MapFromMachineUser", (name, uid_inner, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn map_to_machine_user(&self, uid_outer: u32) -> Result<(String, dbus::Path<'static>, u32), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "MapToMachineUser", (uid_outer, ))
    }

    fn map_from_machine_group(&self, name: &str, gid_inner: u32) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "MapFromMachineGroup", (name, gid_inner, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn map_to_machine_group(&self, gid_outer: u32) -> Result<(String, dbus::Path<'static>, u32), dbus::Error> {
        self.method_call("org.freedesktop.machine1.Manager", "MapToMachineGroup", (gid_outer, ))
    }

    fn pool_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Manager", "PoolPath")
    }

    fn pool_usage(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Manager", "PoolUsage")
    }

    fn pool_limit(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.machine1.Manager", "PoolLimit")
    }
}
