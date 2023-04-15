#![cfg(not(target_arch = "wasm32"))]

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
    providers: Vec<Box<dyn CorePlugin>>,
    _libs: Vec<Library>,
}

fn get_plugins(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .filter(|f| match f {
            Ok(f) => {
                f.file_type().unwrap().is_file()
                    && f.file_name().to_str().unwrap().ends_with("_client.so")
            }
            Err(_) => false,
        })
        .map(|f| f.unwrap().path())
        .collect()
}

impl Provider {
    pub fn global(index: usize) -> &'static dyn CorePlugin {
        INSTANCE
            .get()
            .expect("Provider is not initialized")
            .providers[index]
            .as_ref()
    }

    pub fn providers() -> &'static [Box<dyn CorePlugin>] {
        &INSTANCE
            .get()
            .expect("Provider is not initialized")
            .providers
    }

    pub fn initialize() {
        let exe = std::env::current_exe().unwrap();
        let exe_dir = exe.parent().unwrap();

        println!("scanning {:?}", exe_dir);
        let lib_paths = get_plugins(exe_dir);

        let mut libs = Vec::<Library>::new();
        let mut providers = Vec::<Box<dyn CorePlugin>>::new();

        for lib_path in lib_paths {
            println!("load {}", lib_path.display());
            unsafe {
                let lib = libloading::Library::new(lib_path).unwrap();

                let res = {
                    let plugin_entry_fn: libloading::Symbol<
                        unsafe extern "C" fn() -> Box<dyn CorePlugin>,
                    > = lib.get(PLUGIN_ENTRY_FUNCTION).unwrap();
                    plugin_entry_fn()
                };

                libs.push(lib);
                providers.push(res);
            }
        }

        INSTANCE
            .set(Provider {
                providers,
                _libs: libs,
            })
            .unwrap();
    }
}
