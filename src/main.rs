use image::{ImageBuffer, RgbImage};
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("data_notime.txt").expect("Something went wrong reading the file");
    let arr = contents
        .split("\n")
        .map(|z| z.parse().expect(z))
        .collect::<Vec<f64>>();

    // print!("{:?}", contents);
    let len = arr.len().to_string().parse().unwrap();
    let nvalues = fill(f64::from(1), len);
    // println!("{:?}", nvalues);
    let values = [
        0.1, 0.5, 0.9, 0.2, 0.7, 0.3, 0.6, 0.4, 0.8, 0.1, 0.5, 0.9, 0.2, 0.7, 0.3, 0.6, 0.8,
    ];

    let height = 1000;
    let width = 10000;
    let mut imgbuf = RgbImage::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let valr = (((nvalues[x as usize] + f64::from(1)) * 0.5) * 255.0).floor() as u8;
        let valg = (((nvalues[y as usize] + f64::from(1)) * 0.5) * 255.0).floor() as u8;
        *pixel = image::Rgb([valr, valg, 0]);
    }
    imgbuf.save("visualization.png").unwrap();
}

fn fill(hz: f64, len: u64) -> Vec<f64> {
    let delta_x = 0.002; // 0.002 seconds, 500Hz
    let mut cycle: Vec<f64> = Vec::new();
    let mut counta = f64::from(0);
    for _t in 0..len {
        cycle.push(counta.sin());
        counta = counta + delta_x * hz;
    }
    return cycle;
}
