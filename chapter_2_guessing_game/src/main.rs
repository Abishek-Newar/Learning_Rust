//standard library for input output
use std::io;
//library from rand we use thi sin this code to generate random number
use rand::Rng;
//standard library for ordering used in compare
use std::cmp::Ordering;
fn main() {
    println!("Guess Game!");
    //generating a random number
    let secret_number = rand::thread_rng().gen_range(0..=100);

    //starting a loop so that on wrong guess it gives again cahnce
    loop{
        println!("Enter a value: ");
        //creating a mutable value
    let mut guess: String = String::new();
    //taking input from a user .. using expect as to handle the return of io
    io::stdin().read_line(&mut guess).expect("Unable to read user data");
    //converting string to number
    let guess: u32 = match guess.trim().parse(){
        Ok(num)=> num,
        Err(err)=> {
            println!("Enter a number not a string")
            continue;
        }
    };
    println!("Entered number is: {}",guess);

    //comapring the user input with the random number
    match guess.cmp(&secret_number){
        Ordering::Equal => {
            println!("Guessed right");
            break;
        },
        Ordering::Less => println!("Too Less"),
        Ordering::Greater => println!("Too Large")
    }
    }
}
