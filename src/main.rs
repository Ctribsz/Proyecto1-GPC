mod framebuffer;
mod maze;
mod player;
mod cast_ray; 
mod help_metods;
mod render;  // Añadimos este nuevo módulo
mod controller;
mod constants;
mod map;
mod texture;

use std::time::{Instant, Duration};
use texture::Texture;
use map::render_mini_map;
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

    // Cargar la textura
    let wall_texture = Texture::from_file("assets/texture.png");

    let mode = "3D";
    
    let mut last_time = Instant::now();
    let mut frame_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        process_events(&window, &mut player, &maze, BLOCK_SIZE);

        framebuffer.clear(Color::new(255, 255, 255)); // Limpiar el framebuffer

        if mode == "2D" {
            render2d(&mut framebuffer, &player, &maze, BLOCK_SIZE);
        } else {
            let maze_chars = convert_maze_to_chars(&maze.render());
            render3d(&mut framebuffer, &player, &maze_chars, BLOCK_SIZE, &wall_texture);
        }

        // Dibujar el mini mapa en la esquina superior izquierda
        render_mini_map(
            &mut framebuffer,
            &convert_maze_to_chars(&maze.render()),
            BLOCK_SIZE,
            0.25,         // Escala del 25% del tamaño original
            10,           // Desplazamiento X desde la esquina
            10,           // Desplazamiento Y desde la esquina
        );

        // Update FPS
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_time);

        // Actualiza cada segundo el conteo de frames
        if elapsed >= Duration::new(1, 0) {
            let fps = frame_count;
            frame_count = 0;
            last_time = current_time;
            println!("FPS: {}", fps); // Aquí podrías renderizar el texto en la pantalla
        }


        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();

            window.set_title(&format!("Rust Graphics - FPS: {}", fps));

        window.update_with_buffer(&buffer, framebuffer.width(), framebuffer.height()).unwrap();
        frame_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
