use std::io;
fn main() {

    println!("Guessing Game!");
    small();
}
fn small(){
    println!("Input a number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed : {}",guess);
}
