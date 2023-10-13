use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    print!("Secret number is {}",secret_number);

    loop {
        println!("Pls input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("Your guess is {}",guess);

        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("Pls input a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}