use crate::{color::{Color, RGB}, render::Renderable};
use std::io::Write;

pub trait PPM {
  fn ppm_get_i(&self, i: usize) -> Color;
  fn ppm_get(&self, x: u32, y: u32) -> Color;
  fn ppm_set_i(&mut self, i: usize, c:Color);
  fn ppm_set(&mut self, x: u32, y: u32, c: Color);
  fn ppm_set_alpha(&mut self, x: u32, y: u32, c: Color, a: f32);
}

pub trait WritePPM {
  fn ppm_write<P: Write> (&self, b: &mut P);
}

impl PPM for FrameBuffer {
  fn ppm_get_i(&self, i: usize) -> Color {
    if i*3+2 > self.buf.len() { return 0; }
    let r = self.buf[i*3] as u32;
    let g = self.buf[i*3+1] as u32;
    let b = self.buf[i*3+2] as u32;
    (r << 16) | (g << 8) | b
  }
  fn ppm_get(&self, x: u32, y: u32) -> Color {
    if x >= self.w || y >= self.h { return 0; }
    self.ppm_get_i((y * self.w + x) as usize)
  }

  fn ppm_set_i(&mut self, i: usize, c:Color) { 
    if i*3+2 > self.buf.len() { return; }
    self.buf[i*3]   = (c >> 16) as u8;
    self.buf[i*3+1] = (c >> 8) as u8;
    self.buf[i*3+2] = (c >> 0) as u8;
  }
  fn ppm_set(&mut self, x: u32, y: u32, c: Color) { 
    if x >= self.w || y >= self.h { return; }
    self.ppm_set_i((y * self.w + x) as usize, c);
  }

  fn ppm_set_alpha(&mut self, x: u32, y: u32, c: Color, a: f32) {
    let bg = self.ppm_get(x, y);
    self.ppm_set(x, y, Color::lerp_rgb(bg, c, a));
  }
}

impl WritePPM for FrameBuffer {
  fn ppm_write<P: Write> (&self, b: &mut P) {
      coz::scope!("ppm_write"); 
      writeln!(b, "P6\n{} {}\n255", self.w, self.h).unwrap();
      let bytes: Vec<u8> = self.buf.iter().map(|p| p.to_le_bytes()).flatten().collect(); 
      b.write_all(&bytes).unwrap();
      b.flush().unwrap();
  }
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