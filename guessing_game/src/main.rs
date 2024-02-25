use rand::Rng;
use std::io;
use std::cmp;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut buf = String::new();
        println!("Input your guess");
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        let input: u32 = match buf.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        match input.cmp(&secret) {
            cmp::Ordering::Less => println!("Too low"),
            cmp::Ordering::Greater => println!("Too high"),
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
