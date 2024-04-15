fn main() {
    //References and Borrowing
    let s = String::from("hello");
    let size = size_calc(&s);
    println!("{}", size);
}

fn size_calc(x: &String) -> usize {
    x.len()
}