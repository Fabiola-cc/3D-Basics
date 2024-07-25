use crate::framebuffer::Framebuffer;

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &[Vec<char>]) {
    let cell_size = 4; // Tamaño de cada celda del laberinto

    framebuffer.clear(); // Color de fondo: #0c0b38
    framebuffer.set_background_color(0x0c0b38);

    // Dimensiones del laberinto
    let maze_height = maze.len();
    let maze_width = maze[0].len();

    // Dimensiones del framebuffer
    let framebuffer_width = framebuffer.width();
    let framebuffer_height = framebuffer.height();

    // Calcular desplazamientos para centrar el laberinto
    let offset_x = ((framebuffer_width / cell_size) - maze_width) / 2;
    let offset_y = ((framebuffer_height / cell_size) - maze_height) / 2;

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
                        (x as isize + offset_x as isize) as f32 * cell_size as f32 + dx as f32,
                        (y as isize + offset_y as isize) as f32 * cell_size as f32 + dy as f32,
                    );
                }
            }
        }
    }
}
