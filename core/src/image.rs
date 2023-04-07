pub struct Image {
    pub id: String,
    pub name: String,
    pub size: usize,
}

impl Image {
    pub fn new(id: String, name: String, size: usize) -> Self {
        Self { id, name, size }
    }
}

pub trait ImageProvider: Send {
    fn list(&self) -> Vec<Image>;
}
