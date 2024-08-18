mod framebuffer;
mod from_file;
mod maze_render;
mod player;
mod raycaster;
mod render_extras;

use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use crate::player::Player;
use crate::maze_render::{render_2Dmaze, render_3Dmaze, render_minimap};
use crate::render_extras::{render_welcome_screen, render_game_over_screen, player_reaches_goal};

pub fn process_events(window: &Window, player: &mut Player, maze: &[Vec<char>]) {
    const MOVE_SPEED: f32 = 0.05; // Ajusta la velocidad de movimiento si es necesario
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
        if maze[new_j as usize][new_i as usize] == ' ' || maze[new_j as usize][new_i as usize] == 'g' {
            // Actualizar la posición del jugador solo si no hay colisión
            player.pos.x = new_x;
            player.pos.y = new_y;
        } else {
            println!("Collision at: ({}, {})", new_j, new_i);
        }
    } else {
        println!("Out of bounds: ({}, {})", new_i, new_j);
    }
}

enum GameState {
    Welcome,
    InGame,
    GameOver,
}

fn main() {
    let window_width = 700;
    let window_height = 500;
    let cell_size = 50;

    let framebuffer_width = 700;
    let framebuffer_height = 500;

    let frame_delay = Duration::from_millis(10);

    // // Crear un nuevo stream de salida
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    // // Abrir el archivo de audio
    // let file = File::open("Fluffing-a-Duck.mp3").expect("Failed to open audio file");
    // let file = BufReader::new(file); // Asegurar que el archivo es compatible
    // let source = Decoder::new(file).expect("Failed to decode audio file");
    
    // // Reproducir el audio en un bucle infinito
    // stream_handle.play_raw(source.convert_samples().repeat_infinite()).expect("Failed to play audio");

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Maze Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Crear jugador con posición inicial y ángulo
    let mut player = player::Player::new(1.5, 1.5, 0.785, 1.047);

    let maze = from_file::load_maze("maze.txt").expect("Failed to load maze");

    let mut mode = "3D";
    let mut state = GameState::Welcome; // Inicialmente en la pantalla de bienvenida

    while window.is_open() {
        match state {
            GameState::Welcome => {
                render_welcome_screen(&mut framebuffer);
                window.update_with_buffer(&framebuffer.to_u32_buffer(), framebuffer_width, framebuffer_height).unwrap();
                if window.is_key_down(Key::Space) {
                    state = GameState::InGame;
                }
            }
            GameState::InGame => {
                // Lógica del juego
                process_events(&window, &mut player, &maze);

                framebuffer.clear();
                if mode == "2D" {
                    render_2Dmaze(&mut framebuffer, &maze, &player);
                } else {
                    render_3Dmaze(&mut framebuffer, &maze, &player);
                    render_minimap(&mut framebuffer, &maze, &player, cell_size, 0.3);
                }
                
                if window.is_key_down(Key::M) {
                    mode = if mode == "2D" {"3D"} else {"2D"};
                    println!("{}", mode)
                }
                
                // Mostrar el contenido del framebuffer
                window.update_with_buffer(&framebuffer.to_u32_buffer(), framebuffer_width, framebuffer_height).unwrap();
                
                // Condición para pasar al estado de fin del juego
                if player_reaches_goal(&player, &maze) {
                    state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                render_game_over_screen(&mut framebuffer);
                window.update_with_buffer(&framebuffer.to_u32_buffer(), framebuffer_width, framebuffer_height).unwrap();
                if window.is_key_down(Key::Space) {
                    state = GameState::Welcome; // Volver a la pantalla de bienvenida
                }
            }
        }

        if window.is_key_down(Key::Escape) {
            break;
        }

        std::thread::sleep(frame_delay);
    }
}

