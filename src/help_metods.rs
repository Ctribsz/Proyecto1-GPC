use nalgebra_glm::Vec2;
use crate::maze::Maze;
use crate::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn render_player(player: &Player, framebuffer: &mut Framebuffer, block_size: usize) {
    let screen_x = (player.pos.x * block_size as f32) as usize;
    let screen_y = (player.pos.y * block_size as f32) as usize;

    let player_color = Color::new(188, 167, 246); // Color para el jugador

    // Renderiza al jugador como un solo punto
    framebuffer.set_pixel(screen_x, screen_y, player_color);
}

pub fn find_player_position(maze: &Maze) -> Vec2 {
    for (y, line) in maze.render().iter().enumerate() {
        if let Some(x) = line.chars().position(|c| c == 'p') {
            return Vec2::new(x as f32, y as f32);
        }
    }
    Vec2::new(0.0, 0.0) // Retorno por defecto si no se encuentra 'p'
}

pub fn convert_maze_to_chars(maze: &Vec<String>) -> Vec<Vec<char>> {
    maze.iter().map(|line| line.chars().collect()).collect()
}

