use crate::color::Color;
use crate::framebuffer::{FrameBuffer, PPM};


mod fill;
mod dot;
mod line;
use fill::fill;
use dot::dot;
use line::line;

pub trait Renderable {
  fn fill(&mut self, w: usize, h: usize, color: Color);
  fn dot(&mut self, x: f32, y: f32, color: Color, radius: f32, blur: Option<f32>);
  fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, c: Color);
}

impl Renderable for dyn PPM {
    fn fill(&mut self, w: usize, h: usize, color: Color) {
      fill(self, w, h, color);
    }

    fn dot(&mut self, x: f32, y: f32, color: Color, radius: f32, blur: Option<f32>) {
      dot(self, x, y, color, radius, blur);
    }

    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, c: Color) {
      line(self, x1, y1, x2, y2, c);
    }
}
