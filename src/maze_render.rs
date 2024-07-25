use crate::framebuffer::Framebuffer;
use crate::player::Player;
use nalgebra_glm::Vec2;

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player) {
    let cell_size = 4; // Tamaño de cada celda del laberinto

    framebuffer.clear(); // Color de fondo: #0c0b38
    framebuffer.set_background_color(0x0c0b38);

    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' | '-' | '|' => 0xebdc7f, // Paredes: #ebdc7f
                'p' => 0x00FF00, // Punto de inicio: verde
                'g' => 0xFF0000, // Meta: rojo
                _ => 0x0c0b38,   // Espacios vacíos: color de fondo
            };

            framebuffer.set_current_color(color);

            for dx in 0..cell_size {
                for dy in 0..cell_size {
                    framebuffer.point(
                        x as f32 * cell_size as f32 + dx as f32,
                        y as f32 * cell_size as f32 + dy as f32,
                    );
                }
            }
        }
    }

    // Dibujar al jugador
    render_player(framebuffer, player, cell_size);
}

pub fn render_player(framebuffer: &mut Framebuffer, player: &Player, cell_size: usize) {
    let player_color = 0xFFFFFF; // Blanco
    let player_size = cell_size / 2; // Tamaño del jugador

    framebuffer.set_current_color(player_color);

    // Coordenadas del jugador en el framebuffer
    let player_x = player.pos.x * cell_size as f32;
    let player_y = player.pos.y * cell_size as f32;

    // Dibujar al jugador
    for dx in 0..player_size {
        for dy in 0..player_size {
            framebuffer.point(player_x + dx as f32, player_y + dy as f32);
        }
    }

    // Dibujar la dirección del jugador
    let dir_length = 10.0; // Longitud de la línea que indica la dirección
    let dir_x = player_x + player.angle.cos() * dir_length;
    let dir_y = player_y + player.angle.sin() * dir_length;

    framebuffer.draw_line(
        Vec2::new(player_x, player_y),
        Vec2::new(dir_x, dir_y),
    );
}
