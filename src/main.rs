use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let str_instruction = "Guess the number (1-100)!";
    let str_too_low = "Too low. Try again.";
    let str_too_high = "Calm down! That guess is too high!";
    let str_victory = "Congratulations! You guessed right!";
    let secret_number = rand::thread_rng().gen_range(1, 101);

    cls();
    println!("{:?}", str_instruction);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        cls();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{:?}", str_too_low),
            Ordering::Greater => println!("{:?}", str_too_high),
            Ordering::Equal => {
                println!("{:?}", str_victory);
                break;
            }
        }
    }
}

fn cls() {
    print!("{}[2J", 27 as char);
}
