#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1: User = User {
        username: String::from("ShubyDrive"),
        email: String::from("shub@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    let name: String = user1.username;

    println!("{}", name);
    user1.username = String::from("not_ShubyDrive");

    let user2: User = build_user(String::from("test@example.com"), String::from("tester"));
    println!("rect: {:#?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}