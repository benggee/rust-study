use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Pleaser input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => continue,
        };

        println!("you guessed: {}", guess_number);
        
        if guess_number > secret_number {
            println!("Too big");
        } else if (guess_number < secret_number) {
            println!("Too small");
        } else {
            println!("You win.");
            break;
        }

    }
}
