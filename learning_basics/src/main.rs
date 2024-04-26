fn main() {
    let string = String::from("stressed");
    let reversed_string = reverse_string(&string);
    println!(
        "the reversed string of -{}- is -{}-",
        string, reversed_string
    );
}

//so fucking easy !
fn reverse_string(string: &String) -> String {
    let mut reversed = String::new();
    for letters in string.chars() {
        reversed.insert_str(0, &letters.to_string());
    }
    reversed
}
