use crate::color::Color;
use crate::framebuffer::{FrameBuffer, PPM};


mod fill;
mod dot;
use fill::fill;
use dot::dot;

pub trait Renderable {
  fn fill(&mut self, w: usize, h: usize, color: Color);
}

impl Renderable for dyn PPM {
    fn fill(&mut self, w: usize, h: usize, color: Color) {
        fill(self, w, h, color);
    }
}
