use std::io;

fn main() {
    loop {
        println!("Choose conversion type:");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Enter the temperature to convert:");

        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => println!(
                "{}°F is {}°C",
                temperature,
                fahrenheit_to_celsius(temperature)
            ),
            2 => println!(
                "{}°C is {}°F",
                temperature,
                celsius_to_fahrenheit(temperature)
            ),
            _ => println!("Invalid choice"),
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
