use std::fs;

use utils::normalize;
mod fourier;
mod time;
mod utils;

fn main() {
    let contents =
        fs::read_to_string("EEG_O2_500.txt").expect("Something went wrong reading the file");
    let arr = contents
        .split("\n")
        .map(|z| z.split(";").collect::<Vec<&str>>()[1].parse().expect(z))
        .collect::<Vec<f64>>();

    // let len = arr.len() as u32;

    // In theory, width = range / step
    let test = normalize(arr);
    fourier::fourier(0.002, 10000, 2000, 200, test.clone(), 0.02);
    time::time(5000, 1000, test.clone());
}
