mod framebuffer;
mod from_file;
mod maze_render;
mod player;
mod raycaster;
mod bmp;

use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use crate::player::Player;

pub fn process_events(window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 0.1; // Ajusta la velocidad de movimiento si es necesario
    const ROTATION_SPEED: f32 = std::f32::consts::PI / 30.0; // Ajusta la velocidad de rotación si es necesario
    
    if window.is_key_down(Key::Left) {
        player.angle -= ROTATION_SPEED;
    }
    
    if window.is_key_down(Key::Right) {
        player.angle += ROTATION_SPEED;
    }
    
    if window.is_key_down(Key::Up) {
        player.pos.x += MOVE_SPEED * player.angle.cos();
        player.pos.y += MOVE_SPEED * player.angle.sin();
    }
    
    if window.is_key_down(Key::Down) {
        player.pos.x -= MOVE_SPEED * player.angle.cos();
        player.pos.y -= MOVE_SPEED * player.angle.sin();
    }
}


fn main() {
    let window_width = 700;
    let window_height = 500;
    let cell_size = 50;

    let framebuffer_width = 700;
    let framebuffer_height = 500;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Maze Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Crear jugador con posición inicial y ángulo
    let mut player = player::Player::new(1.5, 1.5, 0.785, 1.047); // Ajustar posición inicial según el laberinto

    // Manejar el resultado de load_maze
    let maze = match from_file::load_maze("maze.txt") {
        Ok(maze) => maze,
        Err(e) => {
            eprintln!("Failed to load maze: {}", e);
            return;
        }
    };

    while window.is_open() {
        // Escuchar entradas
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Renderizar el laberinto y el jugador
        maze_render::render_maze(&mut framebuffer, &maze, &player);
        
        process_events(&window, &mut player);

        // Convertir los datos del framebuffer a un buffer u32
        let buffer = framebuffer.to_u32_buffer();

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
