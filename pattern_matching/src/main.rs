fn main() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Texas);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:#?}", state);
    } else {
        count += 1;
    }
}

#[derive(Debug)]
enum Coin {
    Quarter(UsState),
    Dime,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Texas,
}
