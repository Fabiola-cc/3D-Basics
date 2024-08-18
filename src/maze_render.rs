use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::raycaster::cast_ray;

pub fn render_3Dmaze(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player) {
    let cell_size = 50; // Tamaño de cada celda del laberinto
    let num_rays = framebuffer.width;

    let hh = framebuffer.height as f32 / 2.0; // precalculated half height
    framebuffer.set_background_color(0xece5c8);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.angle - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, maze, player, a, cell_size, false);

        // Cambia el color de acuerdo al tipo de celda que se ha intersectado
        match intersect.impact {
            '+' | '-' | '|' => framebuffer.set_current_color(0xf06a60),
            'p' => framebuffer.set_current_color(0x00FF00), // Verde para punto de inicio
            'g' => framebuffer.set_current_color(0xfb8324),
            _ => framebuffer.set_current_color(0xFFFFFF),   // Blanco para espacios vacíos o no definidos
        }

        let stake_height = framebuffer.height as f32 / intersect.distance;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

        for y in stake_top..stake_bottom {
            if i < framebuffer.width as usize && y < framebuffer.height as usize {
                framebuffer.point(i as f32, y as f32);
            } else {
                println!("Point out of bounds: i = {}, y = {}", i, y);
            }
        }
    }
}

pub fn render_2Dmaze(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player) {
    let cell_size = 50; // Tamaño de cada celda del laberinto

    framebuffer.clear();
    framebuffer.set_background_color(0x0c0b38);

    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' | '-' | '|' => 0xf06a60,
                'p' => 0x00FF00, // Punto de inicio: verde
                'g' => 0xfb8324,
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
    render_player(framebuffer, player, cell_size, &maze);
}

pub fn render_player(framebuffer: &mut Framebuffer, player: &Player, cell_size: usize, maze: &[Vec<char>]) {
    let player_color = 0x4174a3;
    framebuffer.set_current_color(player_color);

    // Coordenadas del jugador en el framebuffer
    let player_x = player.pos.x * cell_size as f32;
    let player_y = player.pos.y * cell_size as f32; 

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

pub fn render_minimap(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player, cell_size: usize, mini_map_scale: f32) {
    let map_offset_x = 0; // Posición X del mini mapa en la pantalla
    let map_offset_y = 0; // Posición Y del mini mapa en la pantalla

    let minimap_cell_size = (cell_size as f32 * mini_map_scale) as usize;

    // Dibujar el laberinto
    for j in 0..maze.len() {
        for i in 0..maze[0].len() {
            let x = (map_offset_x + i * minimap_cell_size) as f32;
            let y = (map_offset_y + j * minimap_cell_size) as f32;

            // Dibuja líneas horizontales y verticales para formar el rectángulo
            for dx in 0..minimap_cell_size as isize {
                for dy in 0..minimap_cell_size as isize {
                    
                    if maze[j][i] != ' ' {
                        // Cambia el color actual a color del rectángulo
                        framebuffer.set_current_color(0xf06a60);
                        framebuffer.point(x + dx as f32, y + dy as f32);
                    } else {
                        framebuffer.set_current_color(0xece5c8);
                        framebuffer.point(x + dx as f32, y + dy as f32);
                    }
                } 
            }
        }
    }


    // Dibujar al jugador
    render_player(framebuffer, player, minimap_cell_size, &maze);
}
