extern crate core;

use std::io;

pub mod gessinggame;
pub mod rust_dock;
pub mod chapter_3;


fn main() {
    loop {
        println!("");
        println!("Choose your Subprogram");
        println!("1 for Exit");
        println!("2 for Gessing-Game");
        println!("3 for Dock and Test");
        println!("4 for a Looping");
        println!("5 for Temprechner");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess {
            1 => break,
            2 => gessinggame::gessing_game(),
            3 => rust_dock::dockumentation(),
            4 => rust_dock::looping(),
            5 => chapter_3::temperatur_calculator(),
            _ => println!("Bitte gib eine verfügbare Nummer ein"),
        }
    }
}
