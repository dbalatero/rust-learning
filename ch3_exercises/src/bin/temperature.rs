use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};

fn main() {
    println!("Temperature converter");
    println!();

    let unit = prompt_for_unit();
    match unit {
        Unit::Celsius => println!("Converting from F to C"),
        Unit::Fahrenheit => println!("Converting from C to F"),
    }
    println!();

    let temperature = prompt_for_temperature();

    let converted: f32 = match unit {
        Unit::Celsius => (temperature - 32.0) / 1.8,
        Unit::Fahrenheit => (temperature * 1.8) + 32.0,
    };

    let short_unit = unit.to_short_unit();

    println!("{temperature} in {unit} is = {converted}° {short_unit}");
}

enum Unit {
    Celsius,
    Fahrenheit,
}

impl Unit {
    fn to_short_unit(&self) -> &str {
        match self {
            Unit::Celsius => "C",
            Unit::Fahrenheit => "F",
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Celsius => write!(f, "celsius"),
            Self::Fahrenheit => write!(f, "fahrenheit"),
        }
    }
}

fn prompt_for_temperature() -> f32 {
    println!("Enter your temperature value");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        return temperature;
    }
}

fn prompt_for_unit() -> Unit {
    println!("What unit do you want to convert to?");
    println!("  Enter 'c' for Celsius");
    println!("  Enter 'f' for Fahrenheit");

    loop {
        println!();
        print!("> ");
        io::stdout().flush().expect("Failed to flush");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let unit = match choice.trim().to_ascii_lowercase().as_str() {
            "c" => Unit::Celsius,
            "f" => Unit::Fahrenheit,
            _ => continue,
        };

        println!();

        return unit;
    }
}
