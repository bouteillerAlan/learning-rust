use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(0..=100);

    println!("Guess the number");
    println!("{} is the secret number ;)", secret);

    loop {
        let mut guess = String::new();

        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut  guess)
            .expect("Failed to read");

        let guess: u32 = guess.trim().parse().expect("Type a number.");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater  => println!("Too big."),
            Ordering::Equal  => {
                println!("You win !");
                break;
            }
        }

        println!("You guessed: {}", guess);

    }

}
