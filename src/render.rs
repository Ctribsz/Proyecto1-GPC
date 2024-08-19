use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze;
use crate::cast_ray::{cast_rays, cast_single_ray};
use crate::help_metods::render_player;
use crate::help_metods::convert_maze_to_chars;
use crate::Color;
use crate::Texture;

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
    texture: &Texture,
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

        // Calcular la coordenada X de la textura
        let wall_x = (intersect.x - intersect.x.floor()) * texture.width as f32;
        let wall_x = wall_x as usize % texture.width;

        for y in stake_top..stake_bottom {
            // Calcular la coordenada Y de la textura
            let texture_y = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * texture.height() as f32) as usize;
            let color_u32 = texture.get_pixel(wall_x, texture_y);
        
            // Extraer los componentes RGB del u32
            let r = ((color_u32 >> 16) & 0xFF) as u8;
            let g = ((color_u32 >> 8) & 0xFF) as u8;
            let b = (color_u32 & 0xFF) as u8;
        
            // Crear un nuevo Color usando los valores RGB
            let color = Color::new(r, g, b);
        
            // Usar el color para dibujar en el framebuffer
            framebuffer.point(i, y, color);
        }
    }
}







