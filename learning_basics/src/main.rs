fn none_some(input: Option<i32>) -> u8 {
    match input {
        None => 0,
        Some(i) => 1,
    }
}

fn main() {
    let x = Some(5);

    println!("value is: {}", none_some(x));
}
