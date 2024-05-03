use std::f64::consts::PI;
use std::fs;
use utils::fill_cos;
mod fourier;
mod time;
mod utils;

fn main() {
    // let contents =
    //     fs::read_to_string("EEG_O2_500.txt").expect("Something went wrong reading the file");
    // let arr = contents
    //     .trim()
    //     .split("\n")
    //     .map(|z| z.split(";").collect::<Vec<&str>>()[1].parse().expect(z))
    //     .collect::<Vec<f64>>();

    let arr = fill_cos(50.0 * 2.0 * PI, 1_000_000, 0.25);

    // let len = arr.len() as u32;

    // In theory, width = range / step
    // let test = normalize(arr);
    time::time(20000, 100, arr.clone());
    fourier::fourier(0.025, 1000, 2000, 250, arr.clone(), 0.25);
}
