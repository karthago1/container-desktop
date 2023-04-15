#![cfg(target_arch = "wasm32")]

use container_core::CorePlugin;
use once_cell::sync::OnceCell;
use simulator_client;

static INSTANCE: OnceCell<Provider> = OnceCell::new();

#[derive(Debug)]
pub struct Provider {
    providers: Vec<Box<dyn CorePlugin>>,
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
        let mut providers = Vec::<Box<dyn CorePlugin>>::new();
        providers.push(simulator_client::initialize());

        INSTANCE.set(Provider { providers }).unwrap();
    }
}
