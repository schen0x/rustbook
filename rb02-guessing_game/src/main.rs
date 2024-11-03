use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // type is i32 as rust default
                                                               // let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // let guess: i64 = guess.trim().parse().expect("Please type a number!"); // rb03? var shadowing; // expect() is not a catch, it just add the string to the panic message
        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
