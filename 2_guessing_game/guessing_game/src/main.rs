use std::io; // std stands for standard library and io is input output operation
             // use keyword is importing the dependency. in c# its using,
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{

        println!("Please input your guess. ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // println!("The secret number is {secret_number}");
        println!("You guessed: {guess}");
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
