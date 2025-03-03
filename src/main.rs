use std::io;

fn read_number() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        match input.trim().parse::<u32>() {
            Ok(num) => {
                return num;
            }
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn main() {
    println!("Please enter counter!");
    let count = read_number();

    let (mut prev, mut next) = (0, 1);

    for _ in 0..count {
        print!("{prev} ");
        (prev, next) = (next, prev + next);
    }
}
