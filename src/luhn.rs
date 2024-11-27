pub fn is_valid(card: &str) -> bool {
    if card.chars().any(|c| !(c.is_digit(10) || c.is_whitespace())) {
        return false;
    }
    let filtered: String = card.chars().filter(|c| c.is_digit(10)).collect();
    if filtered.len() <= 1 {
        return false;
    }
    let sum: u32 = filtered
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut n = c.to_digit(10).unwrap();
            if i % 2 != 0 {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            n
        })
        .sum();
    sum % 10 == 0
}


pub fn main() {
    println!("{}", is_valid("4539 3195 0343 6467"));
    println!("{}", is_valid("8273 1232 7352 0569"));
    println!("{}", is_valid("059"));
    println!("{}", is_valid("055 444 285"));
    println!("{}", is_valid("9999999999 9999999999 9999999999 9999999999"));
}