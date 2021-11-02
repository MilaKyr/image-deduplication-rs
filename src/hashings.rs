use image::imageops::FilterType;
use crate::traits::Converter;
use crate::image::Image;
use std::error::Error;

const AHASH_IMAGE_SIZE: u32 = 8;
const AHASH_FILTER_TYPE: FilterType = FilterType::Gaussian;


pub enum ImageHashing {
    AHash,
    PHash, // TODO
}

pub struct AHash {
    pub converter: ImageConverterParams,
}

impl Default for AHash {
    fn default() -> Self {
        Self {
            converter: ImageConverterParams {
                size: (AHASH_IMAGE_SIZE, AHASH_IMAGE_SIZE),
                filter: AHASH_FILTER_TYPE,
                use_grayscale: true
            }
        }
    }
}

impl AHash {
    fn hash_image(&self, image: Vec<u8>) -> Result<String, Box<dyn Error>> {
        let avg_color = image.iter().map(|v| *v as f32).sum::<f32>() / image.len() as f32;
        let bit_string: String = image
            .iter()
            .map(|v| if *v as f32 > avg_color { "1" } else { "0" } )
            .collect();
        let hex = u64::from_str_radix(&*bit_string, 2)?;
        Ok(format!("{:x}", hex))
    }

    pub  fn add_hashes(&self, images: &mut Vec<Image>) -> Result<Vec<Image>, Box<dyn Error>> {
        for image in images.into_iter() {
            let transformed = self.prepare_image(image);
            let data = transformed.dynamic_image.into_luma8().into_raw();
            let data_hashed = self.hash_image(data)?;
            image.update_hash(data_hashed);
        };
        Ok(images.clone())
    }
}

impl Converter for AHash {
    fn prepare_image(&self, image: &Image) -> Image {
        let (width, high) = &self.converter.size;
        let resized = image.dynamic_image.resize_exact(*width, *high, self.converter.filter.clone());
        let transformed = resized.grayscale();
        Image {
            path: image.path.clone(),
            dynamic_image: transformed,
            hashing: None,
            }
    }
}

pub struct ImageConverterParams {
    pub size: (u32, u32),
    pub filter: FilterType,
    pub use_grayscale: bool,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_image() {
        let image_raw_data: Vec<u8> = vec![1, 2, 3, 4, 5]; // mean is 3
        let hasher = AHash::default();
        let result = hasher.hash_image(image_raw_data).unwrap();
        let expected_result = "3"; // 00011 in hexadecimal system
        assert_eq!(result, expected_result)
    }
}



