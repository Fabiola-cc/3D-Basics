use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn render_welcome_screen(framebuffer: &mut Framebuffer) {
    framebuffer.clear();
    framebuffer.set_background_color(0xdbf7f1); // Color de fondo para la bienvenida
    // Aquí podrías agregar texto, gráficos, etc. 
    // Usa framebuffer.point() para dibujar píxeles y crear una pantalla atractiva
}

pub fn render_game_over_screen(framebuffer: &mut Framebuffer) {
    framebuffer.clear();
    framebuffer.set_background_color(0xe3dbf7); // Color de fondo para el fin del juego
    // Aquí podrías agregar texto o gráficos para la pantalla de fin del juego
    // Usa framebuffer.point() para dibujar
}

pub fn player_reaches_goal(player: &Player, maze: &[Vec<char>]) -> bool {
    let x = player.pos.x as usize;
    let y = player.pos.y as usize;

    if maze[y][x] == 'g' { // Suponiendo que 'g' representa la meta
        return true;
    }
    false
}