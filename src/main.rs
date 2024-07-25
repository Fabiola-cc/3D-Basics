mod framebuffer;
mod from_file;
mod maze_render;
mod bmp;

use std::time::Duration;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Maze Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Manejar el resultado de load_maze
    match from_file::load_maze("maze.txt") {
        Ok(maze) => {
            maze_render::render_maze(&mut framebuffer, &maze);
        }
        Err(e) => {
            eprintln!("Failed to load maze: {}", e);
            return;
        }
    }

    while window.is_open() {
        // Escuchar entradas
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Convertir los datos del framebuffer a un buffer u32
        let buffer = framebuffer.to_u32_buffer();

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}