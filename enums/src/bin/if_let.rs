fn main() {
    let config_max: Option<u8> = Some(3u8);

    if let Some(max) = config_max {
        println!("The max config is {}", max);
    }
}
