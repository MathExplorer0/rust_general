fn main() {
    let string = String::from("stressed");
    let reversed_string = reverse_string(&string);
    println!(
        "the reversed string of -{}- is -{}-",
        string, reversed_string
    );
}

fn reverse_string(string: &String) -> String {
    let reversed = String::new();

    for letters in string.chars() {
        reversed.push_str(letters);
    }

    reversed
}
