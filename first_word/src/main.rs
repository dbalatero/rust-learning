fn main() {
    let s = String::from("hello world");
    let index = first_word_index(&s);

    println!("The first word ends at = {index}");
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
