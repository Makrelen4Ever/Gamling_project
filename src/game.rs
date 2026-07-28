use crate::rendering::{Color, switch_color, clear_terminal};

pub struct Game {
    pub chips: usize,
}

impl Game {
    pub fn init(&mut self) {
        println!("Let's go GAMBLING");

        let mut user_input: String;

        loop {
            user_input = String::new();

            println!("What can we do for you today?");
            println!("You currently have {} chips", self.chips);

            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");

            match user_input.to_lowercase().trim() {
                "blackjack" => {
                    self.black_jack();
                },

                "roulette" => {
                    self.roulette();
                },

                "exit" => {
                    break;
                },

                "clear" => {
                    clear_terminal();
                },

                _ => {
                    clear_terminal();

                    switch_color(Color::RED);
                    println!("Unknown input. Type 'Exit' for exitting.");
                    switch_color(Color::RESET);
                }
            }
        }
    }
}