use std::io;
use std::io::Read;
use std::fs::File;


fn main() {
    let result: Result<String, io::Error> = read_username_from_file();
    println!("Result: {:#?}", result);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
