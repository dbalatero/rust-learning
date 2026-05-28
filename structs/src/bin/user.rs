struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        email,
        username,
    }
}

fn main() {
    println!("Hello");

    let mut user1 = build_user(String::from("db@example.com"), String::from("db"));
    user1.email = String::from("db@new.com");
}
