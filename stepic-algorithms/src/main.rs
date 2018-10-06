use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values = input.split_whitespace().map(|s| &s.parse::<i32>().unwrap()).collect();
}
