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

    let mut lvalues = Vec::new();
    for i in 0..len {
        lvalues.push(nvalues[i as usize] * arr[i as usize])
    }
    lvalues = normalize(lvalues);
    let mut old_x: u32 = 0;
    let mut old_y: u32 = 0;
    let mut points: Vec<[i32; 2]> = Vec::new();

    for x in 0..width {
        // let y = (((lvalues[x as usize] + f64::from(1)) * 0.5) * f64::from(height)).floor() as u32;
        let y = ((lvalues[x as usize]) * f64::from(height)).floor() as u32;
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
        if x < width as i32 && y < height as i32 {
            let some = imgbuf.get_pixel_mut_checked(x as u32, y as u32);
            *some.unwrap() = image::Rgb([255, 255, 255]);
        }
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

fn normalize(arr: Vec<f64>) -> Vec<f64> {
    let max = arr.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min = arr.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let range = max - min;
    let mut normalized = Vec::new();
    for i in arr {
        normalized.push((i - min) / range);
    }
    normalized
}

// Bresenham's line algorithm
fn linedraw(x1: u32, y1: u32, x2: u32, y2: u32) -> Vec<[i32; 2]> {
    let mut points = Vec::new();
    let dy: i32 = y1.abs_diff(y2) as i32;
    let dx: i32 = x1.abs_diff(x2) as i32;
    // Drawing upwards or downwards, to left or to right.
    let step_x: i32 = if x1 < x2 { 1 } else { -1 };
    let step_y: i32 = if y1 < y2 { 1 } else { -1 };

    let mut error = dx - dy;

    let mut t_x: i32 = x1 as i32;
    let mut t_y: i32 = y1 as i32;

    loop {
        points.push([t_x, t_y]);
        if t_x == x2 as i32 && t_y == y2 as i32 {
            break;
        }
        let double_error = 2 * error;
        if double_error > -dy {
            error -= dy;
            t_x += step_x;
        }
        if double_error < dx {
            error += dx;
            t_y += step_y;
        }
    }
    return points;
}
