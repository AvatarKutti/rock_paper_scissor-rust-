use std::io;
use rand::Rng;

fn main() {

    let choices = ["Rock", "Paper", "Scissor"];

    let mut computer_score = 0;
    let mut user_score = 0;

    let mut winning_score = String::new();

    println!("ROCK PAPER SCISSORS!!");

    println!("Choose your winning score:");

    io::stdin()
            .read_line(&mut winning_score)
            .expect("Failed to read line.");

    let winning_score: u32 = winning_score.trim().parse().expect("Please enter a valid number!");

    println!("Your winning score is: {winning_score}");

    loop {

        if user_score == winning_score {
            println!("You win the game. Congratulations!!");
            break;
        }

        if computer_score == winning_score {
            println!("You lost the game. :(");
            break;
        }

        let mut user_choice = String::new();

        let random_index: u32 = rand::thread_rng().gen_range(0..=2);
        
        let computer_choice: &str = choices[random_index as usize];

        println!("Pick your choice(Rock , Paper, Scissor): ");

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line.");
    
        let user_choice = user_choice.trim();
 
        if user_choice != "Rock" && user_choice != "Paper" && user_choice != "Scissor" {
            println!("Please enter a valid choice..");
            continue;
        } 
        
        if user_choice == "Rock" && computer_choice == "Scissor" {
            user_score += 1;
            println!("Computer picked {computer_choice}");
            println!("You win!");
            continue;
        } else if user_choice == "Paper" && computer_choice == "Rock" {
            user_score += 1;
            println!("Computer picked {computer_choice}");
            println!("You win!");
            continue;
        } else if user_choice == "Scissor" && computer_choice == "Paper" {
            user_score += 1;
            println!("Computer picked {computer_choice}");
            println!("You win!");
            continue;
        } else if user_choice == computer_choice {
            println!("Computer also picked {computer_choice}");
            println!("Draw!");
            continue;
        } else {
            computer_score += 1;
            println!("Computer picked {computer_choice}");
            println!("You lost :(");
            continue;
        }
    }
     
}
