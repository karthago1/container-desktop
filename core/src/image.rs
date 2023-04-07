pub struct Image {
    pub id: String,
    pub name: String,
    pub size: usize,
}

pub trait ImageProvider {
    fn list() -> Vec<Image>;
}
