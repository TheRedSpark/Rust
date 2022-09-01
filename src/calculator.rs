use std::env::args;


pub(crate) fn main_calc() {
    let operator = "+";
    let first: f64 = 6.0;
    let last: f64 = 2.0;
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