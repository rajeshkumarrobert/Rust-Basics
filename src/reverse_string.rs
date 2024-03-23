
pub fn reverse(input: &str) -> String {
    let reversed_string = input.chars().rev().collect();
    println!("The reversed string is {}",reversed_string);
    reversed_string
}