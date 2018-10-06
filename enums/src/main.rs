fn main() {
    let x: i8 = 3;
    let y: Option<i8> = Some(5);

    let sum = x + y.expect("y is none");
    println!("{}", sum);
}
