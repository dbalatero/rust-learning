#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    Some(format!("A state: {state:?}"))
}

fn main() {
    // blah
}
