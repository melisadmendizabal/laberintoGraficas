use raylib::color::Color;
use crate::player::Player;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;


pub fn cast_ray(
    framebuffer: &mut Framebuffer, 
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize
){
    let mut d = 0.0;
    framebuffer.set_current_color(Color::WHITE);
  

    loop{
        
        let cos = d * a.cos();
       

        let sin = d * a.sin();
        let x = (player.pos.x + cos);
        let y = (player.pos.y + sin);

        let i = (x as f32 / block_size as f32) as usize;
        let j = (y as f32 /block_size as f32) as usize;

        if maze[j][i] != ' ' {
            break;

        }

        framebuffer.set_pixel(x as u32, y as u32);

        d += 10.0;
    }

}