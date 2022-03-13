use std::io;

fn main(){
    println!("'Guess The Number' Game!");
    println!("In this game you'll be guessing the secret number between a range of numbers, the lower being inclusive while the higher being exclusive. To start provide the lower and higher limits of the range of numbers.");

    let mut _lower_limit = String::new();
    
    println!("Choose the lower limit (number > 0): ");

    io::stdin().read_line(&mut _lower_limit)
    .expect("Failed to read value");

    let _lower_limit: u32 = match _lower_limit.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
}