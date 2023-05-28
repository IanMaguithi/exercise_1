use std::io as stdio;

pub fn temp_conversion() {
    let mut condition: bool = true;
    while condition {
        // user input: choice of conversion
        println!("Choose conversion:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        let mut choice = String::new();
        stdio::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        // user input: temperature
        println!("Enter temperature:");
        let mut temperature = String::new();
        stdio::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        let temperature: f64 = temperature.trim().parse().expect("Please type a number!");

        // conversion
        if choice == 1 {
            fahrenheit_to_celsius(temperature);
        } else if choice == 2 {
            celsius_to_fahrenheit(temperature);
        } else {
            println!("Invalid choice!");
        }

        // user input: continue or exit
        println!("Continue? (y/n)");
        let mut continue_choice = String::new();
        stdio::stdin()
            .read_line(&mut continue_choice)
            .expect("Failed to read line");
        let continue_choice: char = continue_choice
            .trim()
            .parse()
            .expect("Please type a character!");
        if continue_choice == 'n' {
            println!("Exiting...");
            condition = false;
        } else if continue_choice == 'y' {
            println!("Continuing...");
        } else {
            println!("Invalid choice!");
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) {
    let celsius: f64 = (fahrenheit - 32.0) / 1.8;
    println!("{}°F = {}°C", fahrenheit, celsius);
}

fn celsius_to_fahrenheit(celsius: f64) {
    let fahrenheit: f64 = (celsius * 1.8) + 32.0;
    println!("{}°C = {}°F", celsius, fahrenheit);
}
