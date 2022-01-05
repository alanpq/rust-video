use crate::color::Color;
use crate::framebuffer::{FrameBuffer, PPM};
use crate::Renderable;

pub fn fill(buf: &mut (impl PPM + ?Sized), w: usize, h: usize, color: Color) {
  for i in 0..w*h {
    buf.ppm_set_i(i as usize, color);
  }
}