use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub(crate) fn guessed() {
    println!("Please enter one input");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Unable to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess:{guess}");
        println!("Your secret number is:{secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The Guessed value is smaller"),
            Ordering::Greater => println!("The Guessed value is larger"),
            Ordering::Equal => {
                println!("Congrats the value is matching");
                break;
            }
        }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The Guessed value is smaller"),
            Ordering::Greater => println!("The Guessed value is larger"),
            Ordering::Equal => {
                println!("Congrats the value is matching");
                break;
            }
        }
    }
}
