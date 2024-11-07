pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;
pub struct Display;

impl Display {
    pub fn draw(vram: &[u8]) {
        print!("\x1B[1;1H");
        for y in 0..32 {
            for x in 0..64 {
                print!("{}", if vram[x + y * 64] == 1 { "█" } else { " " });
            }
            println!();
        }
    }
}