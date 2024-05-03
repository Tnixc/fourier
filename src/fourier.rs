use crate::utils::{fill_cos, fill_sin, linedraw};
use image::RgbImage;
use num::complex::{Complex, ComplexFloat};
use std::f64;
use std::f64::consts::PI;

pub fn fourier(width: u32, height: u32, range: u32, signal: Vec<f64>, freq_step: f64) {
    let mut points: Vec<[u32; 2]> = Vec::new();
    let mut temp_points: Vec<[f64; 2]> = Vec::new();
    if width != (range as f64 / freq_step).floor() as u32 {
        println!("step * range != width");
        return;
    }

    // Thing in loop runs once for every frequency step.
    for i in 0..(width as i64) {
        let mut sum = Complex::new(0.0, 0.0);
        let freq = freq_step * i as f64;
        for n in 0..signal.len() {
            sum += Complex::new(
                signal[n] * (2.0 * PI * freq * n as f64).cos() * freq_step,
                signal[n] * -(2.0 * PI * freq * n as f64).sin() * freq_step,
            );
        }
        let abs = sum.abs();
        temp_points.push([i as f64, abs]);
        println!("{:?} Hz,{:?}", freq, abs)
    }

    let mut max = f64::NEG_INFINITY;
    let mut min = f64::INFINITY;

    for t in 0..temp_points.len() {
        if temp_points[t][1] > max {
            max = temp_points[t][1];
        }
        if temp_points[t][1] < min {
            min = temp_points[t][1];
        }
    }

    for t in 0..temp_points.len() {
        temp_points[t][1] = (temp_points[t][1] - min) / (max - min) * height as f64;
    }

    let mut x_old = temp_points[0][0].floor() as u32;
    let mut y_old = temp_points[0][1].floor() as u32;

    for t in 1..temp_points.len() {
        let x = temp_points[t][0].floor() as u32;
        let y = temp_points[t][1].floor() as u32;
        points.append(
            &mut linedraw(x_old, y_old, x, y)
                .iter()
                .map(|z| [z[0].abs() as u32, z[1].abs() as u32])
                .collect::<Vec<[u32; 2]>>(),
        );

        x_old = x;
        y_old = y;
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
        println!("{:?}, {:?}", x, y);
        if x < width && y < height {
            let some = imgbuf.get_pixel_mut_checked(x, y);
            *some.unwrap() = image::Rgb([255, 255, 255]);
        }
    }

    imgbuf.save("freq_domain.png").unwrap();
}
