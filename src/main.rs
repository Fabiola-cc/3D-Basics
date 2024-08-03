mod framebuffer;
mod from_file;
mod maze_render;
mod player;
mod raycaster;
mod bmp;

use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use crate::player::Player;

pub fn process_events(window: &Window, player: &mut Player, maze: &[Vec<char>], cell_size: usize) {
    const MOVE_SPEED: f32 = 0.1; // Ajusta la velocidad de movimiento si es necesario
    const ROTATION_SPEED: f32 = std::f32::consts::PI / 30.0; // Ajusta la velocidad de rotación si es necesario

    if window.is_key_down(Key::Left) {
        player.angle -= ROTATION_SPEED;
    }

    if window.is_key_down(Key::Right) {
        player.angle += ROTATION_SPEED;
    }

    let mut new_x = player.pos.x;
    let mut new_y = player.pos.y;

    if window.is_key_down(Key::Up) {
        new_x += MOVE_SPEED * player.angle.cos();
        new_y += MOVE_SPEED * player.angle.sin();
    }

    if window.is_key_down(Key::Down) {
        new_x -= MOVE_SPEED * player.angle.cos();
        new_y -= MOVE_SPEED * player.angle.sin();
    }

    // Convertir las nuevas coordenadas a índices de celda
    let new_i = (new_x as f32).floor() as isize;
    let new_j = (new_y as f32).floor() as isize;

    // Verificar si la nueva posición está dentro de los límites del laberinto
    if new_i >= 0 && new_j >= 0 && new_i < maze[0].len() as isize && new_j < maze.len() as isize {
        // Verificar si la nueva celda es un espacio vacío
        if maze[new_j as usize][new_i as usize] == ' ' {
            // Actualizar la posición del jugador solo si no hay colisión
            player.pos.x = new_x;
            player.pos.y = new_y;
        } else {
            println!("Collision at: ({}, {})", new_i, new_j);
        }
    } else {
        println!("Out of bounds: ({}, {})", new_i, new_j);
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

    let mut mode = "2D";
    while window.is_open() {
        // Escuchar entradas
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" {"3D"} else {"2D"};
            println!("{}", mode)
        }
        process_events(&window, &mut player, &maze, cell_size);


        framebuffer.clear();
        // Renderizar el laberinto y el jugador
        if mode == "2D"{
            maze_render::render_2Dmaze(&mut framebuffer, &maze, &player);
        } else {
            maze_render::render_3Dmaze(&mut framebuffer, &maze, &player);
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
