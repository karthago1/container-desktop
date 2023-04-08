use std::{
    fs,
    path::{Path, PathBuf},
};

use container_core::CorePlugin;
use libloading::Library;
use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<Provider> = OnceCell::new();

static PLUGIN_ENTRY_FUNCTION: &[u8] = b"initialize\0";

#[derive(Debug)]
pub struct Provider {
    provider: Box<dyn CorePlugin>,
    _libs: Vec<Library>,
}

fn get_plugin(dir: &Path) -> PathBuf {
    fs::read_dir(dir)
        .unwrap()
        .find(|f| match f {
            Ok(f) => {
                f.file_type().unwrap().is_file()
                    && f.file_name().to_str().unwrap().ends_with("_client.so")
            }
            Err(_) => false,
        })
        .map(|f| f.unwrap().path())
        .unwrap()
}

impl Provider {
    pub fn global() -> &'static dyn CorePlugin {
        INSTANCE
            .get()
            .expect("Provider is not initialized")
            .provider
            .as_ref()
    }

    pub fn initialize() {
        let exe = std::env::current_exe().unwrap();
        let exe_dir = exe.parent().unwrap();

        println!("scanning {:?}", exe_dir);
        let lib_path = get_plugin(exe_dir);

        println!("load {}", lib_path.display());

        let mut libs = Vec::<Library>::new();
        let plugin = unsafe {
            let lib = libloading::Library::new(lib_path).unwrap();

            let res = {
                let plugin_entry_fn: libloading::Symbol<
                    unsafe extern "C" fn() -> Box<dyn CorePlugin>,
                > = lib.get(PLUGIN_ENTRY_FUNCTION).unwrap();
                plugin_entry_fn()
            };

            libs.push(lib);

            res
        };

        INSTANCE
            .set(Provider {
                provider: plugin,
                _libs: libs,
            })
            .unwrap();
    }
}
