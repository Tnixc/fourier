use std::fs;
mod fourier;
mod time;
mod utils;

fn main() {
    let contents =
        fs::read_to_string("data_notime.txt").expect("Something went wrong reading the file");
    let arr = contents
        .split("\n")
        .map(|z| z.parse().expect(z))
        .collect::<Vec<f64>>();

    let len = arr.len() as u32;

    // time::time(5000, 1000, len, arr);
    fourier::fourier(0.002, 1000, 500, arr)
}
