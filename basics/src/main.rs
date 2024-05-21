use std::collections::HashMap;

fn main() {
    let name = String::from("This is a name");
    let value = String::from("This is a value");

    let mut scores = HashMap::new();

    scores.insert(name, value);
}
