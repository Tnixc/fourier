use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    println!("{:?}", contents)
}
