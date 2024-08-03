use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &[Vec<char>],
    player: &Player,
    a: f32,
    cell_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;
    
    framebuffer.set_current_color(0xFFFFFF);
    
    let maze_width = maze[0].len();
    let maze_height = maze.len();

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        
        let x = ((player.pos.x + cos) * cell_size as f32) as isize;
        let y = ((player.pos.y + sin) * cell_size as f32) as isize;

        // Asegurarse de que x y y están dentro de los límites del framebuffer
        if x < 0 || y < 0 || x >= framebuffer.width as isize || y >= framebuffer.height as isize {
            break;
        }

        let i = (x / cell_size as isize) as usize;
        let j = (y / cell_size as isize) as usize;

        // Asegurarse de que i y j están dentro de los límites del laberinto
        if i >= maze_width || j >= maze_height {
          break;
        }

        if draw_line {
          framebuffer.point(x as f32, y as f32);
        }

        if maze[j][i] != ' ' {
            return Intersect {
                distance: d,
                impact: maze[j][i],
            };
        }

        d += 0.3; // Ajustar la resolución del incremento de d para mayor precisión
    }

    // En caso de no encontrar ninguna colisión, devolver un valor por defecto
    Intersect {
        distance: d,
        impact: ' ',
    }
}