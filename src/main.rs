use std::io;
pub mod gessinggame;



fn main() {
    loop {
        println!("");
        println!("Choose your Subprogram");
        println!("1 for Exit");
        println!("2 for Gessing-Game");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //println!("You guessed: {guess}");

        match guess {
            1 => break,
            2 => gessinggame::gessing_game(),
            13..=19 => println!("A teen"),
            _ => println!("Bitte gib eine verfügbare Nummer ein"),
        }
    }
}
