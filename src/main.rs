extern crate core;

use std::io;

pub mod gessinggame;
pub mod rust_dock;
pub mod chapter_3;
pub mod chapter_4;
pub mod chapter_5;


fn _main() {
    let mut foo = String::new();
    std::io::stdin().read_line(&mut foo).expect("fail");
    dbg!(foo);
}


fn main() {
    loop {
        println!("");
        println!("Choose your Subprogram");
        println!("1 for Exit");
        println!("2 for Gessing-Game");
        println!("3 for Dock and Test");
        println!("4 for a Looping");
        println!("5 for Temprechner");
        println!("6 for Fibonatchi");
        println!("7 for Ownership");
        println!("8 for Chapter 5");
        println!("9 for Area");

        /*        let mut guess = String::new();

                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };*/

        let guess: i32 = 9;
        match guess {
            1 => break,
            2 => gessinggame::gessing_game(),
            3 => rust_dock::dockumentation(),
            4 => rust_dock::looping(),
            5 => chapter_3::temperatur_calculator(),
            6 => chapter_3::fibonacci(),
            7 => chapter_4::ownership(),
            8 => chapter_5::chapter_5(),
            9 => chapter_5::area(),
            _ => println!("Bitte gib eine verfügbare Nummer ein"),
        }
        
        break;
    }
}
