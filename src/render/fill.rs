use crate::color::Color;
use crate::framebuffer::FrameBuffer;
use crate::Renderable;
impl Renderable for FrameBuffer {
  fn fill(&mut self, color: Color) {
    for i in 0..self.w*self.h {
      self.ppm_set_i(i as usize, color);
    }
  }
}