mod hashings;
mod traits;
mod image;

#[cfg(test)]
mod tests {
    use crate::hashings::AHash;
    use crate::image::Image;
    use image::io::Reader as ImageReader;
    use std::path::PathBuf;
    use image::DynamicImage;

    fn read_images(names: &Vec<&str>) -> Option<Vec<Image>> {
        let mut images = vec![];
        for image_name in names.into_iter() {
            let image = ImageReader::open(image_name)
                .ok()?
                .decode()
                .ok()?;
            images.push(Image {
                path: PathBuf::from(image_name),
                dynamic_image: image,
                hashing: None
            });
        }
        Some(images)
    }

    #[test]
    fn test_image_loading(){
        let image_paths = vec![
            "Alyson_Hannigan_200512.jpg",
            "Alyson_Hannigan_200512_300.jpg",
            "Alyson_Hannigan_200512_300_1.jpg",
            "dog.jpeg"
        ];
        let opt_images = read_images(&image_paths);
        match opt_images {
            Some(mut images) => {
                let a_hash = AHash::default();
                let hashed_images = a_hash.add_hashes(&mut images).unwrap();
                assert_eq!(hashed_images.len(), image_paths.len());
                assert!(!matches!(hashed_images[0].dynamic_image, DynamicImage::ImageLuma8{ .. }));
            },
            None => assert!(false)
        }
    }
}
