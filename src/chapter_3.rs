use std::io;

pub(crate) fn temperatur_calculator() {
    println!("Gib 1 für F in C");
    println!("Gib 2 für C in F");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().unwrap();


    if guess == 1 {
        let result = f_c();
        println!("In Celsius sind das {result} °C");
    } else if guess == 2 {
        let result = c_f();
        println!("In Fahrenheit sind das {result} °F");
    }
}


fn f_c() -> f64 {
    println!("Gib deine Fahrenheit ein");
    let mut fahrenheit_str = String::new();
    io::stdin()
        .read_line(&mut fahrenheit_str)
        .expect("Failed to read line");
    //println!("Das hast du eingegeben {fahrenheit_str}");
    let fahrenheit_int: f64 = fahrenheit_str.trim().parse().unwrap();
    let celsius: f64 = (fahrenheit_int - 32.0) * (5.0 / 9.0);
    return celsius;
}

fn c_f() -> f64 {
    println!("Gib deine Celsius ein");
    let mut celsius_str = String::new();
    io::stdin()
        .read_line(&mut celsius_str)
        .expect("Failed to read line");
    println!("Das hast du eingegeben {celsius_str}");
    let celsius_int: f64 = celsius_str.trim().parse().unwrap();
    let fahrenheit: f64 = celsius_int * 1.8 + 32.0;
    return fahrenheit;
}