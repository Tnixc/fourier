use crate::utils::{avg_compress, linedraw, normalize};
use image::RgbImage;

pub fn time(width: u32, height: u32, signal: Vec<f64>) {
    let arr = normalize(signal);
    let mut imgbuf = RgbImage::new(width, height);

    let compressed: Vec<f64> = avg_compress(arr, width as u64);

    let mut old_x: u32 = 0;
    let mut old_y: u32 = 0;
    let mut points: Vec<[i32; 2]> = Vec::new();

    for x in 0..width {
        // let y = (((lvalues[x as usize] + f64::from(1)) * 0.5) * f64::from(height)).floor() as u32;
        let y = ((compressed[x as usize]) * f64::from(height)).floor() as u32;
        if y < height {
            // let some = imgbuf.get_pixel_mut_checked(x, y);
            // *some.unwrap() = image::Rgb([255, 255, 255]);
            points.append(&mut linedraw(old_x, old_y, x, y));
            old_x = x;
            old_y = y;
        }
    }

    for point in points {
        let x = point[0];
        let y = point[1];
        // println!("{:?}, {:?}", x, y);
        if x < width as i32 && y < height as i32 {
            let some = imgbuf.get_pixel_mut_checked(x as u32, y as u32);
            *some.unwrap() = image::Rgb([255, 255, 255]);
        }
    }

    imgbuf.save("time_domain.png").unwrap();
}
