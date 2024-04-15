fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    
    let r3 = &mut s; // no problem
    r3.insert_str(0, "bar");
    println!("{}", s);
}