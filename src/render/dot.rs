use crate::color::{Color, RGB};
use crate::framebuffer::{FrameBuffer, PPM};
use crate::Renderable;

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32{
  let x = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
  x.powf(2.0) * (3.0 - 2.0 * x)
}

pub fn dot(buf: &mut (impl PPM + ?Sized), x: f32, y: f32, color: Color, radius: f32, blur: Option<f32>) {
  let min_radius = radius;
  let max_radius = radius + blur.unwrap_or(5.0);
  let (fr,fg,fb) = color.to_rgb();
  let miny = (y - max_radius - 1.0).floor() as u32;
  let maxy = (y + max_radius + 1.0).ceil() as u32;
  let minx = (x - max_radius - 1.0).floor() as u32;
  let maxx = (x + max_radius + 1.0).ceil() as u32;

  for py in miny..maxy {
    let dy = py as f32 - y;
    for px in minx..maxx {
      let dx = px as f32 - x;
      let d = (dy * dy + dx * dx).sqrt();
      let a = smoothstep(max_radius, min_radius, d);
      let (br, bg, bb) = buf.ppm_get(px, py).to_rgb();
      let r = a * fr + (1.0 - a) * br;
      let g = a * fg + (1.0 - a) * bg;
      let b = a * fb + (1.0 - a) * bb;
      buf.ppm_set(px, py, Color::from_rgb(&r, &g, &b));
    }
  }
}