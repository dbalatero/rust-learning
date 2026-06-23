enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Quit => println!("Quit"),
            Self::Move { x, y } => {
                println!("Moving from {} to {}", x, y)
            }
            Self::Write(message) => println!("Writing: {}", message),
            Self::ChangeColor(x, y, z) => println!("Changing color: ({}, {}, {})", x, y, z),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hi"));
    m.call();

    let m2 = Message::Quit;
    m2.call();

    let m3 = Message::Move { x: 10, y: 20 };
    m3.call();

    let m4 = Message::ChangeColor(255, 255, 0);
    m4.call();
}
