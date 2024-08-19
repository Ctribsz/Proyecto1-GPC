use crate::player::Player;
use crate::help_metods::convert_maze_to_chars;
use crate::maze::Maze;
use minifb::{Key, Window};


pub fn process_events(window: &Window, player: &mut Player, maze: &crate::maze::Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 0.05;
    const ROTATION_SPEED: f32 = std::f32::consts::PI / 10.0;

    let maze_grid = convert_maze_to_chars(&maze.render());

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }

    let mut new_x = player.pos.x;
    let mut new_y = player.pos.y;

    if window.is_key_down(Key::Up) {
        let center_offset = block_size as f32 / 2.0;
        new_x = player.pos.x + player.a.cos() * MOVE_SPEED;
        new_y = player.pos.y + player.a.sin() * MOVE_SPEED;

        let grid_x = ((new_x * block_size as f32 + center_offset) / block_size as f32).floor() as usize;
        let grid_y = ((new_y * block_size as f32 + center_offset) / block_size as f32).floor() as usize;

        if grid_x >= maze_grid[0].len() || grid_y >= maze_grid.len() || maze_grid[grid_y][grid_x] != ' ' && maze_grid[grid_y][grid_x] != 'p' {
        } else {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    if window.is_key_down(Key::Down) {
        let center_offset = block_size as f32 / 2.0;
        new_x = player.pos.x - player.a.cos() * MOVE_SPEED;
        new_y = player.pos.y - player.a.sin() * MOVE_SPEED;

        let grid_x = ((new_x * block_size as f32 + center_offset) / block_size as f32).floor() as usize;
        let grid_y = ((new_y * block_size as f32 + center_offset) / block_size as f32).floor() as usize;


        if grid_x >= maze_grid[0].len() || grid_y >= maze_grid.len() || maze_grid[grid_y][grid_x] != ' ' && maze_grid[grid_y][grid_x] != 'p' {
        } else {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
}
