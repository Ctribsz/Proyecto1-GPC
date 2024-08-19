#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
}

impl Framebuffer {
    
    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        // Asume que las coordenadas ya están validadas y simplemente establece el píxel
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| color.to_u32()).collect()
    }
    
    // Función clear para resetear el framebuffer
    pub fn clear(&mut self, color: Color) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let white = Color::new(255, 255, 255); // Color blanco para inicializar
        let buffer = vec![white; width * height];
        Self { width, height, buffer }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_buffer(&self) -> &Vec<Color> {
        &self.buffer
    }

}