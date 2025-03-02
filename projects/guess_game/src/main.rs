use rand;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Please input your guess.");

    let secret_num = rand::random_range(1..=100);

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input !");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too high"),
        }
    }
}
