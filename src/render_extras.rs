use crate::framebuffer::Framebuffer;
use crate::player::Player;

use image::{DynamicImage, GenericImageView, Rgba};

pub fn render_welcome_screen(framebuffer: &mut Framebuffer) {
    let welcome_image = image::open("welcome.png").unwrap();
    framebuffer.clear();
    framebuffer.set_background_color(0xdbf7f1); // Color de fondo para la bienvenida

    // Renderizar la imagen de bienvenida en el centro de la pantalla
    let screen_center_x = (framebuffer.width / 2) as f32;
    let screen_center_y = (framebuffer.height / 2) as f32;
    let image_center_x = (welcome_image.width() / 2) as f32;
    let image_center_y = (welcome_image.height() / 2) as f32;

    for y in 0..welcome_image.height() {
        for x in 0..welcome_image.width() {
            let pixel = welcome_image.get_pixel(x, y);
            let Rgba(data) = pixel;

            // Si el canal alfa es mayor que cero, renderiza el píxel
            if data[3] > 0 {
                let color = ((data[0] as u32) << 16) | ((data[1] as u32) << 8) | (data[2] as u32);
                framebuffer.set_current_color(color);
                framebuffer.point(
                    screen_center_x - image_center_x + x as f32,
                    screen_center_y - image_center_y + y as f32,
                );
            }
        }
    }
}

pub fn render_game_over_screen(framebuffer: &mut Framebuffer) {
    let game_over_image = image::open("adios.png").unwrap();
    framebuffer.clear();
    framebuffer.set_background_color(0xe3dbf7); // Color de fondo para el fin del juego

    // Renderizar la imagen de fin del juego en el centro de la pantalla
    let screen_center_x = (framebuffer.width / 2) as f32;
    let screen_center_y = (framebuffer.height / 2) as f32;
    let image_center_x = (game_over_image.width() / 2) as f32;
    let image_center_y = (game_over_image.height() / 2) as f32;

    for y in 0..game_over_image.height() {
        for x in 0..game_over_image.width() {
            let pixel = game_over_image.get_pixel(x, y);
            let Rgba(data) = pixel;

            // Si el canal alfa es mayor que cero, renderiza el píxel
            if data[3] > 0 {
                let color = ((data[0] as u32) << 16) | ((data[1] as u32) << 8) | (data[2] as u32);
                framebuffer.set_current_color(color);
                framebuffer.point(
                    screen_center_x - image_center_x + x as f32,
                    screen_center_y - image_center_y + y as f32,
                );
            }
        }
    }
}

pub fn player_reaches_goal(player: &Player, maze: &[Vec<char>]) -> bool {
    let x = player.pos.x as usize;
    let y = player.pos.y as usize;

    if maze[y][x] == 'g' { // Suponiendo que 'g' representa la meta
        return true;
    }
    false
}