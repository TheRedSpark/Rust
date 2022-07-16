use std::io;
//mod gessinggame;



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
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            13..=19 => println!("A teen"),
            _ => println!("Bitte gib eine verfügbare Nummer ein"),
        }
    }
}
