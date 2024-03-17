use image::DynamicImage;

#[derive(Clone)]
pub struct Page {
    pub image: DynamicImage,
    pub name: String,
}

impl Page {

    pub fn new(name : String, image : DynamicImage) -> Self {
        Self {
            image: image,
            name: name,
        }
    }
}