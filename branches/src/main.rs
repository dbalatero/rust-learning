fn main() {
    let number = 7;

    if number != 0 {
        println!("Number was non-zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
