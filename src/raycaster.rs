use crate::framebuffer::Framebuffer;
use crate::player::Player;
use nalgebra_glm::Vec2;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &[Vec<char>],
    player: &Player,
    a: f32,
    block_size: usize,
) {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as isize;
        let y = (player.pos.y + sin) as isize;

        // convert our coordinates in pixels to indices in the maze array
        let i = x / block_size as isize;
        let j = y / block_size as isize;

        // Check if i and j are within bounds
        if i < 0 || i >= maze[0].len() as isize || j < 0 || j >= maze.len() as isize {
            return;
        }

        // if the current item is a wall, we break the loop
        if maze[j as usize][i as usize] != ' ' {
            return;
        }

        framebuffer.point(x as f32, y as f32);

        d += 10.0;
    }
}
