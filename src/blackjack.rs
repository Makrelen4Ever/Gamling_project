use crate::game::Game;
use crate::cards::{Card, Deck, Hand};
use crate::rendering::{Color, switch_color, clear_terminal};

impl Game
{
    pub fn black_jack(&mut self) {
        clear_terminal();

        //Initialization
        let mut deck: Deck = Deck::new();
        let mut player_hand: Hand = Hand::new();
        let mut dealer_hand: Hand = Hand::new();

        println!("Welcome to black jack");

        player_hand.draw_cards(2, &mut deck);
        dealer_hand.draw_cards(2, &mut deck);

        let mut player_busted: bool = false;
        let mut player_bet: usize = 0;

        let mut user_input: String;

        //Pregame, storing bets
        println!("Enter your bet:");
        loop {
            switch_color(Color::RESET);

            user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");
    
            let parse_result = user_input
                .trim()
                .parse();

            if parse_result.is_ok()
            {
                player_bet = parse_result.unwrap();

                if player_bet > self.chips
                {
                    player_bet = self.chips;
                    switch_color(Color::GREEN);
                    println!("All in!");
                }else {
                    switch_color(Color::GREEN);
                    println!("You bet: {0}", player_bet);
                }

                break;
            }else {
                clear_terminal();
                switch_color(Color::RED);
                println!("Enter your bet");
                continue;
            }
        }

        //In game: hitting, standing and doubling down
        loop {
            user_input = String::new();

            switch_color(Color::RESET);
            println!("Dealer hand:");
            dealer_hand.render_hand(false);

            print!("\n");

            dealer_hand.calculate_hand();
            println!("Dealer total value: {0}", dealer_hand.cards[0].val);
            
            println!("Your hand:");
            player_hand.render_hand(true);

            print!("\n");

            player_hand.calculate_hand();
            println!("Your total value: {0}", player_hand.value);

            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");

            match user_input.to_lowercase().trim() {
                "hit" => {
                    clear_terminal();

                    player_hand.draw_cards(1, &mut deck);
                    player_hand.calculate_hand();

                    if player_hand.value > 21
                    {
                        player_busted = true;

                        println!("You busted with a: ");
                        player_hand.cards[player_hand.cards.len() - 1].render();
                        player_hand.value = 0;

                        print!("\n");

                        std::thread::sleep(std::time::Duration::from_millis(500));

                        break;
                    }
                },

                "stand" => {
                    break;
                },

                "double down" => {
                    player_hand.draw_cards(1, &mut deck);
                    player_hand.calculate_hand();

                    player_bet *= 2;

                    if player_hand.value > 21
                    {
                        player_busted = true;

                        println!("You busted with a: ");
                        player_hand.cards[player_hand.cards.len() - 1].render();
                        player_hand.value = 0;

                        print!("\n");

                        std::thread::sleep(std::time::Duration::from_millis(500));

                        break;
                    }

                    break;
                },

                _ => {
                    clear_terminal();

                    switch_color(Color::RED);
                    println!("Unknown input. Type 'Hit', or 'stand' in order to progress.");
                    switch_color(Color::RESET);
                }
            }
        }

        clear_terminal();

        //Post game, draw dealers cards, and check who won
        dealer_hand.calculate_hand();

        if !player_busted
        {
            while dealer_hand.value < 17 {
                dealer_hand.draw_cards(1, &mut deck);
                dealer_hand.calculate_hand();
            }
        }

        print!("\n");

        dealer_hand.render_hand(true);
        println!("\nDealer total value: {0}", dealer_hand.value);
        
        print!("\n");

        player_hand.render_hand(true);
        println!("\nPlayer total value: {0}", player_hand.value);

        if ((player_hand.value > dealer_hand.value && player_hand.value <= 21) || dealer_hand.value > 21) && !player_busted
        {
            switch_color(Color::GREEN);

            println!("\nYou won!");
            self.chips += player_bet;

        }else if dealer_hand.value > player_hand.value{

            switch_color(Color::RED);
            println!("\nDealer won");

            if(self.chips >= player_bet)
            {
                self.chips -= player_bet;
            }else {
                self.chips = 0;
                println!("You overly dense doorframe, you hit the bottom and tried to dig down further. Get your shit together man, the datatype can't go negative smh.");
            }

        }else {

            println!("\n Push!");
        
        }

        switch_color(Color::RESET);
        print!("\n");

    }
}