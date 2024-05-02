use crate::utils::{avg_compress, fill_cos, fill_sin, linedraw};
use image::RgbImage;
use std::f64;

pub fn fourier(delta_x: f64, width: u32, height: u32, range: u32, arr: Vec<f64>, step: f64) {
    let mut points: Vec<[u32; 2]> = Vec::new();
    let mut old_x: u32 = 0;
    let mut old_y: u32 = 0;

    if width != (range as f64 / step).floor() as u32 {
        println!("step * range != width");
        return;
    }
    // Thing in loop runs once for every step.
    let mut freq = step;
    for i in 0..((range as f64 / step).floor() as i64) {
        let x = ((i as f64 / (range as f64 / step)) * width as f64).floor() as u32;
        // println!("x: {:?} | freq: {:?}", x, freq);

        // old_x = x;
        freq += step;
    }

    let mut max = 0;
    for point in points.iter() {
        if point[1] > max {
            max = point[1];
        }
    }

    for point in points.iter_mut() {
        point[1] = (point[1] as f64 * 0.5).floor() as u32;
    }

    for p in points.iter_mut() {
        p[1] = height - p[1];
    }

    let mut imgbuf = RgbImage::new(width, height);
    // let mut temp_y_arr: Vec<f64> = Vec::new();

    // // for x in 0..width {}
    // let compressed = avg_compress(temp_y_arr, width as u64);

    // for x in 0..width {
    //     points[x as usize][0] = compressed[x as usize].abs().floor() as u32;
    // }

    for point in points {
        let x = point[0];
        let y = point[1];
        if x < width && y < height {
            let some = imgbuf.get_pixel_mut_checked(x, y);
            *some.unwrap() = image::Rgb([255, 255, 255]);
        }
    }

    imgbuf.save("freq_domain.png").unwrap();
}
