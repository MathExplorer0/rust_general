#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) {
        println!("True");
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    rect1.width();
}
