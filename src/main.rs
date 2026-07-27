mod rendering;

use rand::{RngExt, rng};
use rendering::{Color, clear_terminal, switch_color};

mod cards;
use cards::{Card, Deck};

fn main() {
    clear_terminal();

    let mut game: Game = Game { chips: 0 };
    game.init();
}

struct Game {
    chips: i32,
}

impl Game {
    fn init(&mut self) {
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
                    switch_color(&Color::RED);
                    println!("Unknown input. Type 'Exit' for exitting.");
                    switch_color(&Color::RESET);
                }
            }
        }
    }

    fn black_jack(&mut self) {
        clear_terminal();

        let mut deck: Deck = Deck::new();
        let mut player_hand: Vec<Card> = Vec::new();
        let mut player_total_hand: usize = 0;

        let mut dealer_hand: Vec<Card> = Vec::new();
        let mut dealer_total_hand: usize = 0;

        println!("Welcome to black jack");

        let mut drawn_card: &mut Card = &mut deck.cards[rng().random_range(0..51)];

        for i in 0..2 {
            while drawn_card.has_been_dealt {
                drawn_card = &mut deck.cards[rng().random_range(0..51)];
            }
    
            drawn_card.has_been_dealt = true;
            player_hand.push(*drawn_card);
        }

        for i in 0..2 {
            while drawn_card.has_been_dealt {
                drawn_card = &mut deck.cards[rng().random_range(0..51)];
            }
    
            drawn_card.has_been_dealt = true;
            dealer_hand.push(*drawn_card);
        }
        
        let mut user_input: String;
        loop {
            user_input = String::new();

            println!("Dealers hand:");
            for card in &dealer_hand {
                card.render();
                print!(" ");
            }

            println!("\n Dealer total value: {dealer_total_hand} \n\n");

            println!("Your hand:");
            for card in &player_hand {
                card.render();
                print!(" ");
            }

            print!("\n");
            println!("Your total value: {player_total_hand}");

            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");

            match user_input.to_lowercase().trim() {
                "hit" => {
                    clear_terminal();

                    while drawn_card.has_been_dealt {
                        drawn_card = &mut deck.cards[rng().random_range(0..51)];
                    }
            
                    drawn_card.has_been_dealt = true;
                    player_hand.push(*drawn_card);

                    player_total_hand = 0;
                    for card in &player_hand
                    {
                        player_total_hand += card.val;
                    }

                    if player_total_hand > 21
                    {
                        println!("You busted with a: ");
                        drawn_card.render();
                        print!("\n");

                        std::thread::sleep(std::time::Duration::from_millis(500));

                        break;
                    }
                },

                "stand" => {
                    break;
                },

                _ => {
                    clear_terminal();

                    switch_color(&Color::RED);
                    println!("Unknown input. Type 'Hit', or 'stand' in order to progress.");
                    switch_color(&Color::RESET);
                }
            }
        }

        dealer_total_hand = 0;
        for card in &dealer_hand {
            dealer_total_hand += card.val;
        }

        while dealer_total_hand < 17 {
            while drawn_card.has_been_dealt {
                drawn_card = &mut deck.cards[rng().random_range(0..51)];
            }
    
            drawn_card.has_been_dealt = true;
            dealer_hand.push(*drawn_card);
        
            dealer_total_hand = 0;
            for card in &dealer_hand {
                dealer_total_hand += card.val;
            }
        }

        print!("\n\n\n");

        println!("Dealers hand:");
        for card in &dealer_hand {
            card.render();
            print!(" ");
        }

        print!("\n");

    }

    fn roulette(&mut self) {
        self.chips -= 10;
        clear_terminal();
    }
}
