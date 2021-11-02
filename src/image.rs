use image::DynamicImage;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug)]
pub enum ImageError{
    HashNotFound
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::HashNotFound => write!(f, "There is no hash for the given image")
        }
    }
}


impl std::error::Error for ImageError {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Image {
    pub path: PathBuf,
    pub dynamic_image: DynamicImage,
    pub hashing: Option<String>,
}

impl Image {
    pub fn humming_distance(&self, other: &Image) -> Result<i32, ImageError> {
        let mut distance = 0;
        match (&self.hashing, &other.hashing) {
            (Some(hash), Some(other_hash)) => {
                let bites_vec: Vec<_> = hash.as_bytes()
                    .iter()
                    .zip(other_hash.as_bytes().iter())
                    .collect();
                for (first_bite, second_bite) in bites_vec.iter() {
                    if first_bite != second_bite {
                        distance += 1;
                    }
                }
                Ok(distance)
            },
            _ => return Err(ImageError::HashNotFound)
        }
    }

    pub fn update_hash(&mut self, hashing: String) {
        self.hashing = Some(hashing);
    }
}