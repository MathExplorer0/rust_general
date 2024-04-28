#[derive(Debug)]
enum IpType {
    V4,
    V6,
}

fn main() {
    let _four = IpType::V4;
    let six = IpType::V6;

    print_type(six);
}

fn print_type(input: IpType) {
    println!("the type is: {:?}", input);
}
