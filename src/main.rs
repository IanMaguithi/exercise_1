mod christmas_lyrics;
mod fibonacci;
mod temp_conversion;

fn main() {
    let mut condition: bool = true;
    while condition {
        // user input: choice of question
        println!("Choose question:");
        println!("1. Temperature conversion");
        println!("2. Fibonacci");
        println!("3. Christmas lyrics");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        // choice of question
        if choice == 1 {
            temp_conversion::temp_conversion();
        } else if choice == 2 {
            fibonacci::fibonacci();
        } else if choice == 3 {
            christmas_lyrics::lyrics();
        } else {
            println!("Invalid choice!");
        }

        // user input: continue or exit
        println!("Continue? (y/n)");
        let mut continue_choice = String::new();
        std::io::stdin()
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
