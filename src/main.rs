use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let mut heads = 0;
    let mut tails = 0;

    println!("Who are you?");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    let user_name = user_name.trim();

    println!("Hello, {}!", user_name);

    println!("Tossing a coin...");

    for round in 1..=3 {
        let toss: bool = rng.gen();
        if toss {
            println!("Round {}: Heads", round);
            heads += 1;
        } else {
            println!("Round {}: Tails", round);
            tails += 1;
        }
    }

    println!("Heads: {}, Tails: {}", heads, tails);
}
