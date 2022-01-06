use crate::color::{Color, RGB};
use crate::framebuffer::{FrameBuffer, PPM};
use crate::Renderable;
use std::mem::swap;

pub fn line(buf: &mut (impl PPM + ?Sized), x0: f32, y0: f32, x1: f32, y1: f32, c: Color) {
    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;
    
    let steep = (y1-y0).abs() > (x1-x0).abs();

    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let grad = match dx == 0.0 {
        true  => 1.0,
        false => dy / dx
    };

    let xend = x0.round();
    let yend = y0 + grad * (xend - x0);
    let xgap = 1.0 - (x0 + 0.5).fract();
    let xpxl1 = xend;
    let ypxl1 = yend.floor();
    if steep {
        buf.ppm_set_alpha(ypxl1 as u32, xpxl1 as u32, c, (1.0 - yend.fract()) * xgap);
        buf.ppm_set_alpha(ypxl1 as u32 + 1, xpxl1 as u32, c, (1.0 - yend.fract()) * xgap);
    } else {
        buf.ppm_set_alpha(xpxl1 as u32, ypxl1 as u32, c, (1.0 - yend.fract()) * xgap);
        buf.ppm_set_alpha(xpxl1 as u32, ypxl1 as u32+1, c, (1.0 - yend.fract()) * xgap);
    }
    let mut intery = yend + grad;

    let xend = x1.round();
    let yend = y1 + grad * (xend - x1);
    let xgap = 1.0 - (x1 + 0.5).fract();
    let xpxl2 = xend;
    let ypxl2 = yend.floor();
    if steep {
        buf.ppm_set_alpha(ypxl2 as u32, xpxl2 as u32, c, (1.0 - yend.fract()) * xgap);
        buf.ppm_set_alpha(ypxl2 as u32+1, xpxl2 as u32, c, (1.0 - yend.fract()) * xgap);
    } else {
        buf.ppm_set_alpha(xpxl2 as u32, ypxl2 as u32, c, (1.0 - yend.fract()) * xgap);
        buf.ppm_set_alpha(xpxl2 as u32, ypxl2 as u32+1, c, (1.0 - yend.fract()) * xgap);
    }

    if steep {
        for x in xpxl1 as u32 + 1..xpxl2 as u32 - 1 {
            buf.ppm_set_alpha(intery.floor() as u32  , x, c, 1.0 - intery.fract());
            buf.ppm_set_alpha(intery.floor() as u32+1, x, c, intery.fract());
            intery = intery + grad;
        }
    } else {
        for x in xpxl1 as u32 + 1..xpxl2 as u32 - 1 {
            buf.ppm_set_alpha(x, intery.floor() as u32  , c, 1.0 - intery.fract());
            buf.ppm_set_alpha(x, intery.floor() as u32+1, c, intery.fract());
            intery = intery + grad;
        }
    }


}