use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze;
use crate::cast_ray::{cast_rays, cast_single_ray};
use crate::help_metods::render_player;
use crate::help_metods::convert_maze_to_chars;
use crate::Color;

pub fn render2d(framebuffer: &mut Framebuffer, player: &Player, maze: &maze::Maze, block_size: usize) {
    maze::render_framebuffer(framebuffer, maze, block_size);
    render_player(player, framebuffer, block_size);
    let maze_chars = convert_maze_to_chars(&maze.render());
    cast_rays(framebuffer, &maze_chars, player, block_size);
}

pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    let num_rays = framebuffer.width();  // Un rayo por cada columna en la pantalla
    let hw = framebuffer.width() as f32 / 2.0;  // Mitad del ancho
    let hh = framebuffer.height() as f32 / 2.0; // Mitad de la altura

    let distance_to_projection_plane = hw / (player.fov / 2.0).tan();

    framebuffer.clear(Color::new(200, 200, 200)); // Fondo gris claro

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_single_ray(framebuffer, maze, player, block_size, a, false);

        let distance_to_wall = intersect.distance;

        // Calcular la altura de la stake
        let stake_height = (distance_to_projection_plane / distance_to_wall).clamp(1.0, hh) * block_size as f32;

        // Calcular las posiciones superior e inferior para dibujar la stake
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height() as f32) as usize;

        // Dibujar la stake directamente en el framebuffer
        let stake_color = Color::new(255, 0, 0); // Color rojo
        for y in stake_top..stake_bottom {
            framebuffer.point(i, y, stake_color);
        }
    }
}






