use iced::widget::image;
use once_cell::sync::OnceCell;

static ICONS: OnceCell<Vec<image::Handle>> = OnceCell::new();

pub static ICON_CLONE: usize = 0;
pub static ICON_CONTAINER: usize = 1;
pub static ICON_DELETE: usize = 2;
pub static ICON_DONE: usize = 3;
pub static ICON_HOURGLASS: usize = 4;
pub static ICON_IMAGE: usize = 5;
pub static ICON_PLAY: usize = 6;
pub static ICON_SETTINGS: usize = 7;
pub static ICON_STOP: usize = 8;
pub static ICON_SAVE_AS: usize = 9;

const ICON_BYTES: &[&'static [u8]] = &[
    include_bytes!("../../icons/clone.png"),
    include_bytes!("../../icons/container.png"),
    include_bytes!("../../icons/delete.png"),
    include_bytes!("../../icons/done.png"),
    include_bytes!("../../icons/hourglass.png"),
    include_bytes!("../../icons/image.png"),
    include_bytes!("../../icons/play.png"),
    include_bytes!("../../icons/settings.png"),
    include_bytes!("../../icons/stop.png"),
    include_bytes!("../../icons/save-as.png"),
];

pub fn icon(index: usize) -> &'static image::Handle {
    &ICONS.get().unwrap()[index]
}

pub fn load_icons() {
    /*let dir = if cfg!(target_arch = "wasm32") {
        "icons/".to_string()
    } else {
        format!("{}/icons/", env!("CARGO_MANIFEST_DIR"))
    };*/

    let icons = ICON_BYTES
        .iter()
        .map(|&icon| image::Handle::from_memory(icon))
        .collect();
    ICONS.set(icons).unwrap();
}
