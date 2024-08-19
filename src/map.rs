use crate::Framebuffer;
use crate::Color;
pub fn render_mini_map(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    scale: f32,
    offset_x: usize,
    offset_y: usize,
) {
    let mini_block_size = (block_size as f32 * scale) as usize;

    for (y, line) in maze.iter().enumerate() {
        for (x, &cell) in line.iter().enumerate() {
            let color = match cell {
                '|' | '-' | '+' => Color::new(51, 51, 76), // Gris para paredes
                ' ' => Color::new(255, 192, 203),          // Rosa para caminos
                'p' => Color::new(0, 255, 0),              // Verde para inicio
                'g' => Color::new(255, 0, 0),              // Rojo para meta
                _ => Color::new(255, 255, 255),            // Blanco para cualquier otra cosa
            };

            // Dibujar el bloque a escala reducida
            for dy in 0..mini_block_size {
                for dx in 0..mini_block_size {
                    framebuffer.set_pixel(
                        offset_x + x * mini_block_size + dx,
                        offset_y + y * mini_block_size + dy,
                        color,
                    );
                }
            }
        }
    }
}
