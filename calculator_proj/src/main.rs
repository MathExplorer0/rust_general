use std::io;

fn main() {
    println!("Hello and welcome to the MATH calculator !");
    loop {
        println!("Please enter fist digit: ");
        let i1 = get_input();
        println!("input x: {}", i1);
        println!("Please enter second digit: ");
        let i2 = get_input();
        println!("input x: {}", i2);

        println!("0-to exit | 1-to add | 2-to subtract | 3-to multiply | 4-to divide");
        let choice = get_input();
        let mut output = 0.0; 
        if choice > 4.0 {
            println!("out of bound!");
            continue;
        } else if choice == 0.0 {
            break;
        } else if choice == 1.0 {
            output = addition(i1, i2);
        } else if choice == 2.0 {
            output = subtraction(i1, i2);
        } else if choice == 3.0 {
            output = multiplication(i1, i2);
        } else if choice == 4.0 {
            output = division(i1, i2);
        }
        println!("the final result is: {}", output);
    }
}

fn get_input() -> f64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("ERROR");
    let buffer: f64 = buffer.trim().parse().expect("please type a number!");
    buffer
}

fn addition(x: f64, y: f64) -> f64 { x + y }

fn subtraction(x: f64, y: f64) -> f64 { x - y }

fn multiplication(x: f64, y: f64) -> f64 { x * y } 

fn division(x: f64, y: f64) -> f64 { x / y } 