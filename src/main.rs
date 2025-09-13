use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..10);

    println!("The secret number is : {}", secret_number);

    loop {
            println!("Please inpu your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input the number");
                continue;
            }
        };

        println!("you guessd: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}