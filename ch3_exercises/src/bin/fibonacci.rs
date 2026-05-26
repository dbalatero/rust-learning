use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: u32 = args.get(1).and_then(|arg| arg.parse().ok()).unwrap_or(10);

    println!("Calculating fibonacci number {n}...");

    let result = fib(n);

    println!("Got result = {result}");
}

fn fib(n: u32) -> u32 {
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
}
