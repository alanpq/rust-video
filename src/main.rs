mod color;
mod framebuffer;
mod render;

use std::io::stdout;

use color::{Color, RGB};
use framebuffer::{FrameBuffer, PPM, WritePPM};
use render::Renderable;
use rand::prelude::*;

fn main() {
    let mut frame = FrameBuffer::new(50, 50);
    let mut out = stdout();
    let mut rng = rand::thread_rng();
    let mut time: f32 = 0.0;
    loop {
        let w = frame.w;
        let h = frame.h;
        (&mut frame as &mut dyn PPM).fill(w as usize, h as usize, Color::from_rgb(&1.0, &1.0, &1.0));
        (&mut frame as &mut dyn PPM).dot(25.0 + (time/50.0).sin() * 25.0, 25.0 + (time/50.0).cos() * 25.0, 0, 1.0, Some(0.1));
        frame.ppm_write(&mut out);
        time += 1.0;
    }    
}
