use std::f64::consts::PI;
use std::fs;
use utils::fill_cos;
mod fourier;
mod time;
mod utils;

fn main() {
    // let contents =
    //     fs::read_to_string("data_notime.txt").expect("Something went wrong reading the file");

    // let arr = contents
    //     .split("\n")
    //     .map(|z| z.parse().expect(z))
    //     .collect::<Vec<f64>>();

    let contents = fs::read_to_string("e.txt").expect("Something went wrong reading the file");

    let arr = contents
        .trim()
        .split("\n")
        .map(|z| z.parse().expect(z))
        .collect::<Vec<f64>>();

    // let mut content: String = String::new();

    // for _ in 0..100_00 {
    //     for _ in 0..100 {
    //         content += "10\n"
    //     }
    //     for _ in 0..100 {
    //         content += "-10\n"
    //     }
    // }

    // let _ = fs::write("e.txt", content);

    time::time(5000, 1000, arr.clone());
    fourier::fourier(1000, 2000, 250, arr.clone(), 0.25);
}
