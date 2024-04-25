//WIP

fn main() {
    println!("Pattern drawing program");
    //draw_expanding(3);
    draw_pyramid(3);
}

// fn draw_expanding(x: u8) {
//     println!("Expending pattern with {x} lines");
//     for i in 0..x {
//         for _j in 0..i + 1 {
//             print!("*");
//         }
//         println!();
//     }
// }
fn draw_pyramid(x: u8) {
    println!("Pyramid pattern with {} lines", x);

    let mut space = 0;

    if x % 2 == 0 {
        space = x / 2;
    } else {
        space = (x / 2) + 1;
    }

    for i in 0..x {
        for _j in 0..space {
            print!(" ");
        }
        space -= 1;
        for _j in 0..i + 1 {
            print!("*");
        }
        println!();
    }
}
