mod color;
mod framebuffer;

use std::io::stdout;

use color::{Color, RGB};
use framebuffer::{FrameBuffer};
use rand::prelude::*;

fn main() {
    let mut frame = FrameBuffer::new(50, 50);
    let mut out = stdout();
    let mut rng = rand::thread_rng();
    for x in 0..frame.w {
        for y in 0..frame.h {
            frame.ppm_set(x, y, Color::from_rgb(&rng.gen(), &rng.gen(), &rng.gen()));
        }
    }
    loop {
        for x in 0..frame.w {
            for y in 0..frame.h {
                let (r,g,b) = frame.ppm_get(x, y).to_rgb();
                let r = r * 0.9;
                let g = g * 0.9;
                let b = b * 0.9;
                frame.ppm_set(x, y, Color::from_rgb(&r, &g, &b));
            }
        }
        frame.ppm_write(&mut out);
    }    
}
