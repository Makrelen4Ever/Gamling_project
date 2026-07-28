use crate::game::Game;
use crate::cards::{Card, Deck, Hand};
use crate::rendering::{Color, switch_color, clear_terminal};

impl Game
{
    pub fn roulette(&mut self) {
        self.chips -= 10;
        clear_terminal();
    }

}