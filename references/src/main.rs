// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//
//     println!("The length of '{s1}' is {len}");
// }
//
// fn calculate_length(str: &str) -> usize {
//     str.len()
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
//
//     println!("s = {s}");
// }
//
// fn change(str: &mut String) {
//     str.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     let s1 = &mut s;
//     let s2 = &mut s;
//
//     println!("{s1}, {s2}");
// }

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
