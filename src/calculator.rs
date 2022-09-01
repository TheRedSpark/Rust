use std::env::args;
use std::io;

pub(crate) fn main_calc() {

    println!("Gib die erste Zahl ein.");
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("Failed to read line");
    let first: f64 = first.trim().parse().unwrap();
    println!("Gib die Rechenoperation an.");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator: &str = operator.trim();
    println!("Gib die 2 Zahl an.");
    let mut last = String::new();
    io::stdin().read_line(&mut last).expect("Failed to read line");
    let last: f64 = last.trim().parse().unwrap();
    match operator {
        "*" => {
            let result = multipy(first, last);
            println!("Das Ergebnis ist {}", result);
        }
        "+" => {
            let result = addition(first, last);
            println!("Das Ergebnis ist {}", result);
        }
        "-" => {
            let result = division(first, last);
            println!("Das Ergebnis ist {}", result);
        }
        "/" => {
            let result = subtraction(first, last);
            println!("Das Ergebnis ist {}", result);
        }
        _ =>  println!("Error Operator"),
    }
}

fn multipy(first: f64, last: f64) -> f64 {
    let result: f64 = first * last;
    return result;
}

fn addition(first: f64, last: f64) -> f64 {
    let result: f64 = first + last;
    return result;
}

fn division(first: f64, last: f64) -> f64 {
    let result: f64 = first / last;
    return result;
}

fn subtraction(first: f64, last: f64) -> f64 {
    let result: f64 = first - last;
    return result;
}