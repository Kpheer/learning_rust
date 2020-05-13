fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("The email of user1 is {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 //auto completion
    };

    println!("The email of user2 is {}", user2.email);

    // This actually works
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(2, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0)
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

