pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn main() {
    let input = "hello";
    let reversed = reverse(input);
    println!("The reversed '{}' word is '{}'", input, reversed);
}