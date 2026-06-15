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
    let mut user1 = build_user(String::from("db@example.com"), String::from("db"));
    user1.email = String::from("db@new.com");

    println!("Hello {0}", user1.email);

    let user2 = User {
        email: String::from("googoo@gaga.com"),
        ..user1
    };

    println!("Also hi to {0}", user2.email);
}
