use crate::framebuffer::{Framebuffer, Color};
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub x: f32,  // Agrega este campo para almacenar la coordenada x
}


pub fn cast_rays(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    block_size: usize,
) {
    let ray_count = 20; // Número de rayos que queremos lanzar
    let angle_step = player.fov / ray_count as f32;

    for i in 0..ray_count {
        let ray_angle = player.a - (player.fov / 2.0) + (i as f32 * angle_step);
        let _intersection = cast_single_ray(framebuffer, maze, player, block_size, ray_angle, false);
    }
}

pub fn cast_single_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    block_size: usize,
    ray_angle: f32,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;
    let ray_color = Color::new(255, 0, 0); // Color rojo para visibilidad

    let center_offset = block_size as f32 / 2.0;

    loop {
        let cos = d * ray_angle.cos();
        let sin = d * ray_angle.sin();
        let x = ((player.pos.x * block_size as f32 + center_offset) + cos) as usize;
        let y = ((player.pos.y * block_size as f32 + center_offset) + sin) as usize;

        if draw_line {
            framebuffer.point(x, y, ray_color);
        }

        let i = x / block_size;
        let j = y / block_size;

        if i >= maze[0].len() || j >= maze.len() || maze[j][i] != ' ' && maze[j][i] != 'p' {
            let wall_x = (x as f32 / block_size as f32) - (i as f32);
            return Intersect {
                distance: d,
                impact: maze[j][i],
                x: wall_x,  // Calcula la posición relativa en la pared
            };
        }

        d += 0.1;
    }
}


