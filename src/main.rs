extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut heads = 0;
    let mut tails = 0;

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