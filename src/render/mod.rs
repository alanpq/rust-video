use crate::color::Color;
pub trait Renderable {
  fn fill(&mut self, color: Color);
}

mod fill;