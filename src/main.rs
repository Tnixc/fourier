use image::{ImageBuffer, RgbImage};
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("data_notime.txt").expect("Something went wrong reading the file");
    let arr = contents
        .split("\n")
        .map(|z| z.parse().expect(z))
        .collect::<Vec<f64>>();

    let len = arr.len().to_string().parse().unwrap();
    let nvalues = fill_sin(f64::from(1), len);

    let height = 1000;
    let width = 10000;
    let mut imgbuf = RgbImage::new(width, height);

    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let valr = (((nvalues[x as usize] + f64::from(1)) * 0.5) * 255.0).floor() as u8;
    //     let valg = (((nvalues[y as usize] + f64::from(1)) * 0.5) * 255.0).floor() as u8;
    //     *pixel = image::Rgb([valr, valg, 0]);
    // }

    // for term in nvalues {imgbuf.get_pixel_mut(x, y)}
    for x in 0..width {
        let y = (((nvalues[x as usize] + f64::from(1)) * 0.5) * f64::from(height)).floor() as u32;
        let mut some = imgbuf.get_pixel_mut_checked(x, y);
        *some.unwrap() = image::Rgb([255, 255, 255]);
    }
    imgbuf.save("visualization.png").unwrap();
}

fn fill_sin(hz: f64, len: u64) -> Vec<f64> {
    let delta_x = 0.002; // 0.002 seconds, 500Hz
    let mut cycle: Vec<f64> = Vec::new();
    let mut counta = f64::from(0);
    for _t in 0..len {
        cycle.push(-counta.sin());
        counta = counta + delta_x * hz;
    }
    return cycle;
}
