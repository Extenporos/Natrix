use console::Style; // styles for the CLI
use std::{io::{self, Write}}; // io library
use crate::handler;
pub fn shell() {
    let welcome = Style::new().bright().cyan().bold(); //welcome message style
    // comment lol
    // printing the welcome message
    println!("{}", welcome.apply_to("Natrix v1.1.0 'Rust Re-Write'"));
    loop {
        print!("$ "); // prints the shells start
        io::stdout().flush().unwrap(); // idk how it works but makes an input

        let mut input: String = String::new(); //string declaration
        io::stdin().read_line(&mut input).unwrap(); //reads the line
        if input == "\n" {
            continue;
        }
        let command: &str = input.trim(); // saves the readed line of the input
        if command == "exit" { //built-in exit command
            break;
        }
        handler::handle(command); // sends the saved command to the handler
    }
}
