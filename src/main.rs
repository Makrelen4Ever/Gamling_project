mod rendering;
mod game;
mod cards;
mod blackjack;
mod roulette;

use rendering::{Color, clear_terminal, switch_color};
use game::{Game};

fn main() {
    clear_terminal();

    let mut game: Game = Game { chips: 1000 };
    game.init();
}