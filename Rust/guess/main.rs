use std::io;

fn main(){

    loop {
        println!("Guess the number!");
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin::read_line(&mut guess) //passing in address of said value
            .expect("Failed to read line");
    
        // println!("You guessed: {guess}");
        println!("You guessed: {}", guess);
        break
    }

    let guess = 5;
    let guess = 6;
    println!("{}", guess);
   
}