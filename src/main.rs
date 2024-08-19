mod framebuffer;
mod maze;
mod player;
mod cast_ray; 
mod help_metods;
mod render;  // Añadimos este nuevo módulo
mod controller;
mod constants;

use controller::process_events;
use help_metods::{find_player_position, convert_maze_to_chars};
use framebuffer::{Framebuffer, Color};
use minifb::{Key, Window, WindowOptions};
use player::Player;
use render::{render2d, render3d};  // Importamos las funciones de renderizado
use constants::{WIDTH, HEIGHT, BLOCK_SIZE};


fn main() {

    let mut maze = maze::Maze::new(WIDTH, HEIGHT);
    maze.generate();
    let maze_chars = convert_maze_to_chars(&maze.render());

    let framebuffer_width = (WIDTH * 3 + 1) * BLOCK_SIZE;
    let framebuffer_height = (HEIGHT * 2 + 1) * BLOCK_SIZE;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let player_pos = find_player_position(&maze); // Posición inicial del jugador
    let player_angle = 0.0; // Ángulo inicial de visión
    let player_fov = std::f32::consts::PI / 3.0; // FOV de 60 grados

    let mut player = Player::new(player_pos, player_angle, player_fov);

    let mut window = Window::new(
        "Rust Graphics - Maze Example",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut mode = "3D";  // Modo inicial

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        process_events(&window, &mut player, &maze.render(), BLOCK_SIZE); // Pasar el laberinto como Vec<String>

        framebuffer.clear(Color::new(255, 255, 255)); // Limpia el framebuffer

        if mode == "2D" {
            render2d(&mut framebuffer, &player, &maze, BLOCK_SIZE);
        } else {
            render3d(&mut framebuffer, &player, &maze_chars, BLOCK_SIZE);
        }

        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();

        window.update_with_buffer(&buffer, framebuffer_width, framebuffer_height).unwrap();
    }
}
