use std::io;

fn main() {
    let mut input = String::new();
    let mut x: u128 = 0;
    let mut y: u128 = 1;
    let mut z: u128 = 0;

    println!("enter input: ");
    io::stdin().read_line(&mut input).expect("ERROR READING");

    let input: u32 = input.trim().parse().expect("ERROR TRIM AND PARSE");

    for _i in 0..input + 1 {
        z = x + y;
        y = x;
        x = z;
        println!("{}", z);
    }
}
