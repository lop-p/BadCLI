use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    // check if the argument says "play"
    if args[1] == "play" {
        println!("What is the capitol of New Jersey?");
        // let's check user input and see if it's correct
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim() == "Trenton" {
            println!("Correct!");

            println!("What is the capitol of New York?");
            let mut ans2wer = String::new();
            std::io::stdin().read_line(&mut ans2wer).expect("Failed to read line");
            if ans2wer.trim() == "Albany" {
                println!("Correct!");

                println!(" Congrats! You have completed the game!\nYou win, absolutely nothing!");
            } else {
                println!("Incorrect!");
                println!("The correct answer is Albany");
            }
        } else {
            println!("Incorrect!");
            println!("The correct answer is Trenton");
        }
    } else {
        println!("bad CLI game");
    }

}
