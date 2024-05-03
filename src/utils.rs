use std::f64::consts::PI;

pub fn fill_sin(hz: f64, len: u64, delta_x: f64) -> Vec<f64> {
    let mut cycle: Vec<f64> = Vec::new();

    for t in 0..len {
        cycle.push((t as f64 * (1.0 / hz)).sin());
    }

    return cycle;
}

pub fn fill_cos(hz: f64, len: u64, delta_x: f64) -> Vec<f64> {
    let mut cycle: Vec<f64> = Vec::new();
    let mut counta: f64 = 0.0;

    for t in 0..len {
        cycle.push((t as f64 * (1.0 / hz)).cos());
    }

    return cycle;
}

pub fn normalize(arr: Vec<f64>) -> Vec<f64> {
    let max = arr.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min = arr.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let range = max - min;
    let mut normalized = Vec::new();
    for i in arr {
        normalized.push((i - min) / range);
    }
    return normalized;
}

// Bresenham's line algorithm
pub fn linedraw(x1: u32, y1: u32, x2: u32, y2: u32) -> Vec<[i32; 2]> {
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

pub fn normalize_with_x(arr: Vec<[u32; 2]>) -> Vec<[u32; 2]> {
    let mut max = u32::MAX;
    let min = 0;

    for i in arr.iter() {
        if i[1] > max {
            max = i[1]
        }
        // if i[1] < min {
        //     min = i[1]
        // }
    }
    let mut normalized = Vec::new();
    for i in 0..arr.len() {
        normalized.push([arr[i][0], (arr[i][1] - min) / (max - min)]);
    }

    return normalized;
}

pub fn avg_compress(arr: Vec<f64>, target_len: u64) -> Vec<f64> {
    let len = arr.len() as u64;
    let mut compressed = Vec::new();
    let step = len / target_len;
    for i in 0..target_len {
        let mut sum: f64 = 0.0;
        for j in 0..step {
            sum += arr[(i * step + j) as usize];
        }
        compressed.push(sum / step as f64);
    }
    return compressed;
}
