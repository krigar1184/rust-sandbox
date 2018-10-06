use std::io;


fn main() {
    loop {
        println!("Input c:");

        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Failed to read line");

        let c: f64 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("C is {}", c);
        println!("F is {}", to_fahrenheit(c));
        println!("Check: {} is {}", c, to_celsius(to_fahrenheit(c)));
    }
}

fn to_celsius(value: f64) -> f64 {
    (value - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(value: f64) -> f64 {
    9.0 * value / 5.0 + 32.0
}
