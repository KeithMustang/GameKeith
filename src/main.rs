mod gb;
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
const HEIGHT: usize =160;
const WIDTH: usize =144;

fn u8_rgb(r: u8, g:u8, b:u8) -> u32 {
    let (r,g,b) =(r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
} //probably will be unused store color scheme in an array in hex

fn main() {
    let color: [u32;4] = [0x08182,0x346856,0x88c070,0xe0f8d0];
    let mut buffer: Vec<u32> = vec![color[3]; WIDTH * HEIGHT];
    
    let mut f = File::open("game.gb").unwrap();
    let mut file = Vec::new();
    f.read_to_end(&mut file).unwrap();

    println!("{:?}",file);
    let mut window = Window::new(
    "GameKeith",
    WIDTH,
    HEIGHT,
    WindowOptions::default(),)
    .unwrap_or_else(|e|{panic!("{}",e);});

    let mut game_keith = gb::GameKeith{
        memory:[0x00; 0xFFFF],
        a:0x00,
        f:0x00,
        b:0x00,
        c:0x00,
        d:0x00,
        e:0x00,
        h:0x00,
        l:0x00,
        s:0x00,
        p:0x00,
        pc:0x0100,
        instruction:0x0000,
        rom:file,
    };
 
    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape){

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
        .unwrap();
    }
}
