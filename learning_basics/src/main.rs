use std::thread::sleep;
use std::time::Duration;

struct Position(f64, f64, f64);

//helix equations implementation in rust

fn main() {
    let is_open = true;
    let mut count: f64 = 0.0;
    let mut position_of_point = Position(0.0, 0.0, 0.0);

    while is_open {
        position_of_point.0 = count.cos();
        position_of_point.1 = count.sin();
        position_of_point.2 = count;

        count += 0.1;

        sleep(Duration::from_millis(100));

        println!(
            "x: {:.2}, y: {:.2}, z: {:.2}",
            position_of_point.0, position_of_point.1, position_of_point.2
        );
    }
}
