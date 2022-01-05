use crate::color::Color;
use std::io::Write;

pub trait PPM {
  fn ppm_set(&mut self, x: u32, y: u32, c: Color);
  fn ppm_get(&self, x: u32, y: u32) -> Color;
  fn ppm_write<P:Write> (&self, b:&mut P);
}

pub struct FrameBuffer {
  pub w: u32,
  pub h: u32,
  buf: Vec<u8>,
}

impl FrameBuffer {
  pub fn new(w: u32, h: u32) -> FrameBuffer {
      FrameBuffer {
          w, h,
          buf: vec![0; (w*h*3) as usize]
      }
  }
}

impl PPM for FrameBuffer {
  fn ppm_get(&self, x: u32, y: u32) -> Color {
      let idx: usize = (y * self.h * 3 + x * 3) as usize;
      let r = self.buf[idx] as u32;
      let g = self.buf[idx+1] as u32;
      let b = self.buf[idx+2] as u32;
      (r << 16) | (g << 8) | b
  }
  fn ppm_set(&mut self, x: u32, y: u32, c: Color) { 
      let idx: usize = (y * self.h * 3 + x * 3) as usize;
      self.buf[idx]   = (c >> 16) as u8;
      self.buf[idx+1] = (c >> 8) as u8;
      self.buf[idx+2] = (c >> 0) as u8;
  }
  fn ppm_write<P: Write> (&self, b: &mut P) {
      writeln!(b, "P6\n{} {}\n255", self.w, self.h).unwrap();
      let bytes: Vec<u8> = self.buf.iter().map(|p| p.to_le_bytes()).flatten().collect(); 
      b.write_all(&bytes).unwrap();
      b.flush().unwrap();
  }
}