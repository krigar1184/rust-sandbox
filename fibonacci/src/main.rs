use std::io;


fn main() {
    loop {
        println!("Input n: ");
        let mut first = 1;
        let mut second = 1;
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Not a number");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };

        for _ in 1..n {
            let tmp = second;
            second = first + second;
            first = tmp;
        }

        println!("{} fibonacci number is {}", 
            match n {
                1 => format!("{}st", 1),
                2 => format!("{}nd", 2),
                3 => format!("{}rd", 3),
                _ => format!("{}th", n),
            },
            second
        );
    }
}
