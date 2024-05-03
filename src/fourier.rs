use crate::utils::{fill_cos, fill_sin, linedraw};
use image::RgbImage;
use std::f64;
use std::f64::consts::PI;

pub fn fourier(
    delta_x: f64,
    width: u32,
    height: u32,
    range: u32,
    signal: Vec<f64>,
    freq_step: f64,
) {
    let mut points: Vec<[u32; 2]> = Vec::new();
    let mut temp_points: Vec<[f64; 2]> = Vec::new();

    let mut old_x: u32 = 0;
    let mut old_y: u32 = 0;

    if width != (range as f64 / freq_step).floor() as u32 {
        println!("step * range != width");
        return;
    }
    // Thing in loop runs once for every frequency step.
    let mut freq = 0.0;
    for i in 0..(width as i64) {
        let x = i as u32;

        let mut real_sum = 0.0;
        let mut imag_sum = 0.0;

        for n in 0..signal.len() {
            let real_part = signal[n as usize] * (2.0 * PI * freq * n as f64).cos();
            let imag_part = -signal[n as usize] * (2.0 * PI * freq * n as f64).sin();
            real_sum += real_part * delta_x;
            imag_sum += imag_part * delta_x;
        }

        let magnitude = (real_sum.powi(2) + imag_sum.powi(2)).sqrt();
        println!("{:?} Hz, {:?}", freq, magnitude);
        temp_points.push([x as f64, magnitude]);

        // old_x = x;
        // old_y = y;
        freq += freq_step;
    }

    // let mut max = 0;
    // for point in points.iter() {
    //     if point[1] > max {
    //         max = point[1];
    //     }
    // }

    // for point in points.iter_mut() {
    //     point[1] = (point[1] as f64 * 0.5).floor() as u32;
    // }

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
        println!("{:?}, {:?}", x, y);
        if x < width && y < height {
            let some = imgbuf.get_pixel_mut_checked(x, y);
            *some.unwrap() = image::Rgb([255, 255, 255]);
        }
    }

    imgbuf.save("freq_domain.png").unwrap();
}
