mod color;
mod framebuffer;

use std::io::stdout;

use color::{Color, RGB};
use framebuffer::{FrameBuffer, PPM};
use rand::prelude::*;

fn main() {
    let mut frame = FrameBuffer::new(50, 50);
    let mut out = stdout();
    let mut rng = rand::thread_rng();
    loop {
        for x in 0..frame.w {
            for y in 0..frame.h {
                frame.ppm_set(x, y, Color::from_rgb(&rng.gen(), &rng.gen(), &rng.gen()));
            }
        }
        frame.ppm_write(&mut out);
    }    
}
