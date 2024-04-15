use std::io;

fn main() { 
    println!("Hello and welcome to the MATH calculator !");
    
    let i1 = get_input();
    println!("input: {}", i1);
}

fn get_input() -> u64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("ERROR");
    let buffer: u64 = buffer.trim().parse().expect("please type a number!");
    buffer
}

fn addition(x: u64, y: u64) -> u64 {
    x + y
}

fn subtraction(x: u64, y: u64) -> u64 {
    x - y
}

fn multiplication(x: u64, y: u64) -> u64 {
    x * y
}

fn division(x: u64, y: u64) -> u64 {
    x / y
}