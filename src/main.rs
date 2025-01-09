// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
mod numrs;
use numrs::NdArray;

fn main() {
    // println!("Guess the number!");
    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop {
    //     let mut guess = String::new();
    //     println!("please inmput your guess.");
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("your guess {guess}");
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // Create a zeros array
    let zero_array = NdArray::<f64>::zeros(vec![3, 3]);
    println!("Zero Array: {:?}", zero_array.data)
}
