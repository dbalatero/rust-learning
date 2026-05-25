// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", world!");
//
//     println!("The string is: {s}");
// }

// fn main() {
//     let x = 5;
//     let y = x;
//
//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
//
//     let s1 = String::from("hello");
//     let s2 = s1;
//
//     // This line won't work because the value moved:
//     // println!("The value of s1 is: {s1}");
//     println!("The value of s2 is: {s2}");
//
//     let mut s = String::from("hello");
//     println!("{s}, world!");
//
//     s = String::from("ahoy");
//     println!("{s}, world!");
//
//     let s3 = String::from("hello");
//     let s4 = s3.clone();
//
//     println!("s3 = {s3}, s4 = {s4}");
// }

fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("outer x = {x}");

    let s2 = String::from("s2 hello");
    let s3 = takes_and_gives_back(s2);

    // Fails, due to move of s2 (given back in s3 though)
    // println!("s2 = {s2}");
    println!("s3 = {s3}");

    let haha = String::from("haha");
    let (s4, len) = calculate_length(haha);

    println!("The length of '{s4}' is {len}");
}

fn takes_ownership(str: String) {
    println!("str = {str}");
}

fn makes_copy(n: i32) {
    println!("n = {n}");
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn calculate_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}
