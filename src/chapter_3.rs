use std::io;

pub(crate) fn temperatur_calculator() {
    println!("Gib 1 für F in C");
    println!("Gib 2 für C in F");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Das hast du eingegeben {guess}");
    let my_int: i32 = guess.parse().unwrap();

    if my_int == 1 {
        f_c()
    } else if my_int == 2 {
        c_f()
    }

}

fn f_c() {
    println!("Gib deine Fahrenheit ein");
    let mut fahrenheit_str = String::new();
    io::stdin()
        .read_line(&mut fahrenheit_str)
        .expect("Failed to read line");
    println!("Das hast du eingegeben {fahrenheit_str}");
    let _fahrenheit_int: i32 = fahrenheit_str.parse().unwrap();
}

fn c_f() {
    println!("Gib deine Celsius ein");
    let mut celsius_str = String::new();
    io::stdin()
        .read_line(&mut celsius_str)
        .expect("Failed to read line");
    println!("Das hast du eingegeben {celsius_str}");
    let _celsius_int: i32 = celsius_str.parse().unwrap();
}