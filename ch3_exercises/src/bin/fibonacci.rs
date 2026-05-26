use std::env;

const MAX_N: u64 = 93;
const DEFAULT_N: u64 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: u64 = match args.get(1) {
        Some(arg) => match arg.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: expected an unsigned 64-bit integer, got {arg:?}");
                std::process::exit(1);
            }
        },
        None => DEFAULT_N,
    };

    if n > MAX_N {
        eprintln!("Error: fibonacci number {n} is too large. Max supported value is {MAX_N}.");
        std::process::exit(1);
    }

    println!("Calculating fibonacci number {n}...");

    let result = fib(n);

    println!("Got result = {result}");
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;

            for _ in 2..n {
                let sum = a + b;
                a = b;
                b = sum;
            }

            a + b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_zero() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn fib_one() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn fib_small() {
        assert_eq!(fib(6), 8);
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn fib_larger() {
        assert_eq!(fib(20), 6765);
    }

    #[test]
    fn fib_larger_than_32_bytes() {
        assert_eq!(fib(48), 4807526976);
    }

    #[test]
    fn fib_max_u64_input() {
        assert_eq!(fib(MAX_N), 12200160415121876738);
    }
}
