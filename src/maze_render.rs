use crate::framebuffer::Framebuffer;
use crate::player::Player;
use nalgebra_glm::Vec2;
use crate::raycaster::cast_ray;

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player) {
    let cell_size = 50; // Tamaño de cada celda del laberinto

    framebuffer.clear(); // Color de fondo: #0c0b38
    framebuffer.set_background_color(0x0c0b38);

    let height = maze.len();

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
    render_player(framebuffer, player, cell_size, height, &maze);
}

pub fn render_player(framebuffer: &mut Framebuffer, player: &Player, cell_size: usize, height: usize, maze: &[Vec<char>]) {
    let player_color = 0xFFFFFF; // Blanco
    framebuffer.set_current_color(player_color);

    // Coordenadas del jugador en el framebuffer
    let player_x = player.pos.x * cell_size as f32;
    let player_y = player.pos.y * cell_size as f32; // Invertir eje y

    // Dibujar al jugador como un solo píxel
    framebuffer.point(player_x, player_y);
    
    // draw what the player sees
    let num_rays = 5;
    for i in 0 .. num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.angle - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, cell_size, true);
    }
}
