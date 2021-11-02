use crate::image::Image;

pub trait Converter {
    fn prepare_image(&self, image: &Image) -> Image;
}