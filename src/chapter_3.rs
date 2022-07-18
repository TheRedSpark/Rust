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



    fn f_c() -> f64 {
        println!("Gib deine Fahrenheit ein");
        let mut fahrenheit_str = String::new();
        io::stdin()
            .read_line(&mut fahrenheit_str)
            .expect("Failed to read line");
        //println!("Das hast du eingegeben {fahrenheit_str}");
        let fahrenheit_int: f64 = fahrenheit_str.trim().parse().unwrap();
        let celsius: f64 = (fahrenheit_int - 32.0) * (5.0 / 9.0);
        return celsius.round();
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
        return fahrenheit.round();
    }
}


pub(crate) fn fibonacci() {
    println!("Wie viele Fibonatchi Nummern möchtest du haben?");

    let mut fibonatchi_anzahl = String::new();

    io::stdin()
        .read_line(&mut fibonatchi_anzahl)
        .expect("Failed to read line");

    let fibonatchi_anzahl: i32 = fibonatchi_anzahl.trim().parse().unwrap();
    let mut index = 0;
    let mut fib_1: i128 = 0;
    let mut fib_2: i128 = 1;
    let mut fib_3: i128;
    while index < fibonatchi_anzahl {
        fib_3 = fib_1 + fib_2;
        fib_1 = fib_2;
        fib_2 = fib_3; //todo!("Nummernanzahl fixen");
        index += 1;
        if index == fibonatchi_anzahl {
            println!("Deine Fibonatchi Zahl {fib_3}");
        }
    }
}

