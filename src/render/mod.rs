use crate::color::Color;
use crate::framebuffer::{FrameBuffer, PPM};


mod fill;
mod dot;
use fill::fill;
use dot::dot;

pub trait Renderable {
  fn fill(&mut self, w: usize, h: usize, color: Color);
  fn dot(&mut self, x: f32, y: f32, color: Color, radius: f32, blur: Option<f32>);
}

impl Renderable for dyn PPM {
    fn fill(&mut self, w: usize, h: usize, color: Color) {
      fill(self, w, h, color);
    }

    fn dot(&mut self, x: f32, y: f32, color: Color, radius: f32, blur: Option<f32>) {
      dot(self, x, y, color, radius, blur);
    }
}
