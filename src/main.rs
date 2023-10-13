use std::cmp::Ordering;
use std::io;

use rand::{Rng, thread_rng};

fn main() {
    println!("Let's guess number!");

    let secret_number = thread_rng().gen_range(1,101);
    println!("secret_number is {}",secret_number);

    loop {
        println!("pls input your number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_) =>continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small！"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too Big"),
        }
    }

}
