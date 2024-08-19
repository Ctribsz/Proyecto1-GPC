use image::GenericImageView;

pub struct Texture {
    data: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn from_file(path: &str) -> Texture {
        let img = image::open(path).expect("Failed to load texture");
        let (width, height) = img.dimensions();
        let data = img.to_rgba8()
            .pixels()
            .map(|p| {
                let channels = p.0;
                (channels[0] as u32) << 16 | (channels[1] as u32) << 8 | (channels[2] as u32)
            })
            .collect();

        Texture {
            data,
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.data[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
