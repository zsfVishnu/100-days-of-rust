use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;
use rand::Rng;

fn main()  {
    let mut wins = 0;
    let mut games = 0;
    loop {
        games +=1;
        println!("\n##############################################\n");
        println!("ROCK !! PAPER !! SCISSORS !! LIZARD !! SPOCK !!");
        println!("\n##############################################\n");
        let items = vec!["Rock", "Paper", "Scissors", "Lizard", "Spock"];
        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap().unwrap() + 1;
        wins += play(items.clone(), user_selection);
        println!(" \n");
        println!("Number of games : {}" , games);
        println!("Number of wins : {}" , wins);
        // println!("\n************************** \n");
        println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%% \n");

        let progress = vec!["Play again", "Exit"];
        let choice = Select::with_theme(&ColorfulTheme::default())
            .items(&progress)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap().unwrap();

        if choice.eq(&1) {
            break;
        }
    }
    println!("Bye Bye");
}

fn play(items: Vec<&str>, user_selection: usize) -> u8 {
    let mut c = 0;
    let mut rng = rand::thread_rng();
    let computer_selection = rng.gen_range(1..(items.len()+1));
    println!("Your Choice: {}", items[user_selection -1 ]);
    println!("Computer Choice: {}",items[computer_selection - 1]) ;
    println!();

    if user_selection.eq(&computer_selection) {
        println!("Similar choice! Try again...");
        println!("\n************************** \n");
        return 0;
    }
    let value_vector = vec![13, 14, 21, 25, 32, 34, 42, 45, 51, 53];
    let value_item = (10 * user_selection) + computer_selection;
    println!("**************************");
    if value_vector.contains(&value_item) { println!("{}", "Lucky Try!!!You win!!!!!!");
        c += 1;
    }
    else { println!("Bazinga!!!! You Lose!!!"); }
    println!("**************************");
    c
}




