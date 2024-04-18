use std::io;

fn main() {
    println!("Hello and welcome to the MATH calculator !");
    let mut buffer = String::new();
    loop {
        println!("Please enter fist digit: ");
        let i1 = get_input(&mut buffer);
        println!("input x: {}", i1);
        println!("Please enter second digit: ");
        let i2 = get_input(&mut buffer);
        println!("input x: {}", i2);

        println!("0-to exit | 1-to add | 2-to subtract | 3-to multiply | 4-to divide");
        let choice = get_input(&mut buffer);
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

fn get_input(buffer: &mut String) -> f64 {
    buffer.clear();
    io::stdin().read_line(buffer).expect("ERROR");
    buffer.trim().parse().expect("please type a number!")
}

fn addition(x: f64, y: f64) -> f64 {
    x + y
}

fn subtraction(x: f64, y: f64) -> f64 {
    x - y
}

fn multiplication(x: f64, y: f64) -> f64 {
    x * y
}

fn division(x: f64, y: f64) -> f64 {
    x / y
}
