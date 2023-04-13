use iced::widget::image;
use once_cell::sync::OnceCell;

pub static ICON_CLONE: usize = 0;
pub static ICON_CONTAINER: usize = 1;
pub static ICON_DELETE: usize = 2;
pub static ICON_DONE: usize = 3;
pub static ICON_HOURGLASS: usize = 4;
pub static ICON_IMAGE: usize = 5;
pub static ICON_PLAY: usize = 6;
pub static ICON_SETTINGS: usize = 7;
pub static ICON_STOP: usize = 8;

static ICONS: OnceCell<[image::Handle; 9]> = OnceCell::new();

pub fn icon(index: usize) -> &'static image::Handle {
    &ICONS.get().unwrap()[index]
}

pub fn load_icons() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let icon_names = [
        "clone.png",
        "container.png",
        "delete.png",
        "done.png",
        "hourglass.png",
        "image.png",
        "play.png",
        "settings.png",
        "stop.png",
    ];

    let icons = icon_names.map(|e| image::Handle::from_path(format!("{}/icons/{e}", dir)));
    ICONS.set(icons).unwrap();
}
