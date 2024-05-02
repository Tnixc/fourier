use crate::utils::{fill_cos, fill_sin, linedraw, normalize};
use image::RgbImage;
use std::f64;

pub fn fourier(delta_x: f64, height: u32, range: u32, arr: Vec<f64>) {
    let width = range;

    for x in 0..range {
        let freq = x + 1;
        println!("{:?}", freq)
    }
}
