use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut state = true;
    
    while state {
        println!("please enter the array index");

        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("failed to read the line");
        let index: usize = index.trim().parse().expect("the input is not a number");   
        if index <= a.len()-1 {
            let element = a[index]; 
            println!("the element of the array index {index} is {element}");   
            state = false;
        }else {
            println!("Err the index {index} is out of bound, indexes allowed: 0-{}", a.len()-1);
        } 
    } 
}