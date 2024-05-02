use crate::utils::{avg_compress, fill_cos, fill_sin, linedraw};
use image::RgbImage;
use std::f64;

pub fn fourier(
    from: u32,
    delta_x: f64,
    width: u32,
    height: u32,
    range: u32,
    arr: Vec<f64>,
    x_scale: f64,
) {
    let to = from + range;
    let mut points: Vec<[u32; 2]> = Vec::new();
    let mut old_x: u32 = 0;
    let mut old_y: u32 = 0;

    for x in from..to {
        let freq = (x + 1) as f64 * x_scale;

        let real = fill_cos(freq as f64, range as u64)
            .iter()
            .map(|z| z * delta_x * arr[x as usize])
            .collect::<Vec<f64>>();
        let imag = fill_sin(freq as f64, range as u64)
            .iter()
            .map(|z| z * delta_x * arr[x as usize])
            .collect::<Vec<f64>>();

        let real_area = real.iter().fold(0.0, |a, &b| a + b);
        let imag_area = imag.iter().fold(0.0, |a, &b| a + b);

        let magnitude = f64::sqrt(real_area.powi(2) + imag_area.powi(2));

        let y = ((magnitude) * f64::from(height)).floor() as u32;
        points.push([x, y]);

        let line = linedraw(old_x, old_y, x, y);
        for point in line {
            points.push([point[0].abs() as u32, point[1].abs() as u32]) // goofy hacks
        }

        old_x = x;
        old_y = y;
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
        p[0] = p[0] - from;
        p[1] = height - p[1];
    }

    let mut imgbuf = RgbImage::new(width, height);
    let mut temp_y_arr: Vec<f64> = Vec::new();

    for x in 0..width {}
    let compressed = avg_compress(temp_y_arr, width as u64);

    for x in 0..width {
        points[x as usize][0] = compressed[x as usize].abs().floor() as u32;
    }

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
