pub trait RGB {
  fn lerp_rgb(a: Self, b: Self, t: f32) -> Self;
  fn from_rgb(r: &f32, g: &f32, b: &f32) -> Self;
  fn to_rgb(&self) -> (f32,f32,f32);
}

impl RGB for u32 {
  fn lerp_rgb(a: Self, b: Self, t: f32) -> Self {
    let (ar,ag,ab) = a.to_rgb();
    let (br,bg,bb) = b.to_rgb();
    Self::from_rgb(&(br*t + ar*(1.0-t)), &(bg*t + ag*(1.0-t)), &(bb*t + ab*(1.0-t)))
  }
  fn from_rgb(r: &f32, g: &f32, b: &f32) -> u32 {
      let r = (r*255.0).round() as u32;
      let g = (g*255.0).round() as u32;
      let b = (b*255.0).round() as u32;
      (r << 16) | (g << 8) | b
  }
  fn to_rgb(&self) -> (f32, f32, f32) {
      ((self >> 16) as f32 / 255.0,
       ((self >> 8) & 0xff) as f32 / 255.0,
       (self & 0xff) as f32 / 255.0
      )
  }
}

pub type Color = u32;