// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[0..5];
//     let world = &s[6..11];
//
//     println!("{hello} {world}");
//
//     // try out split/enumerate
//     for (i, word) in s.split(" ").enumerate() {
//         println!("word {i} = {word}");
//     }
//
//     let word = first_word(&s);
//     println!("The first word is = {word}");
// }
//
fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    str
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);

    // s.clear();

    println!("The first word is: {word}");
}
