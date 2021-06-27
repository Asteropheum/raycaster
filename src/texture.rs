use crate::pack_color;

pub struct Texture {
    image_width: u32,
    image_height: u32,
    pub count: u32,
    pub size: u32,
    image: Vec<u32>,
}

impl Texture {
    pub fn new(filename: &str) -> Result<Texture, image::ImageError> {
        let rgba_img = image::open(filename)?
            .as_rgba8()
            .expect("Cannot open texture")
            .to_owned();

        let (width, height) = rgba_img.dimensions();

        let texture_count = (width / height) as usize;
        let texture_size = width as usize / texture_count;

        if width != height * texture_count as u32 {
            return Err(image::ImageError::FormatError(String::from(
                "Texture file doesn't contain enough packed square textures",
            )));
        }

        let mut texture = vec![0; (width * height) as usize];
        let rgba_img = rgba_img.to_vec();

        for j in 0..height {
            for i in 0..width {
                let r = rgba_img[((i + j * width) * 4) as usize];
                let g = rgba_img[((i + j * width) * 4 + 1) as usize];
                let b = rgba_img[((i + j * width) * 4 + 2) as usize];
                let a = rgba_img[((i + j * width) * 4 + 3) as usize];
                texture[(i + j * width) as usize] = pack_color(r, g, b, Some(a));
            }
        }

        Ok(Texture {
            image_width: width as u32,
            image_height: height as u32,
            count: texture_count as u32,
            size: texture_size as u32,
            image: texture,
        })
    }

    pub fn get(&self, i: u32, j: u32, idx: u32) -> Option<u32> {
        self.image
            .get((i + idx * self.size + j * self.image_width) as usize)
            .cloned()
    }

    pub fn get_scaled_column(
        &self,
        texture_id: u32,
        texture_coord: u32,
        column_height: u32,
    ) -> Option<Vec<u32>> {
        let mut column: Vec<u32> = vec![0; column_height as usize];
        for y in 0..column_height {
            column[y as usize] =
                match self.get(texture_coord, (y * self.size) / column_height, texture_id) {
                    Some(pixel) => pixel,
                    None => return None,
                }
        }
        Some(column)
    }
}
