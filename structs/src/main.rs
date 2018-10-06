fn main() {
    let mut user = build_user(String::from("krigar1184@gmail.com"), String::from("krigar1184"));
    let mut user2 = User {
        username: String::from("another_krigar1184"),
        email: String::from("some@example.com"),
        ..user
    };
    println!("{}", user2.username);
    println!("{}", user.username);
    user.username = String::from("Rivfader");
    println!("{}", user.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}

struct Color(u32, u32, u32);
struct Point(i32, i32, i32);
