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
        {
        let w = frame.w;
        let h = frame.h;
            let hw = w as f32 / 2.0;
            let hh = h as f32 / 2.0;

            let frame = &mut frame as &mut dyn PPM;
            frame.fill(w as usize, h as usize, Color::from_rgb(&1.0, &1.0, &1.0));
            frame.dot(hw + (time/50.0).sin() * hw, hh + (time/50.0).cos() * hh, 0, 256.0, Some(10.0));
        }
        frame.ppm_write(&mut out);
        time += 1.0;
        coz::progress!("main loop");
    }    
}
