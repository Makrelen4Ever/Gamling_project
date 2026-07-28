use std::ops::{Add};

use rand::{RngExt, rng};

#[derive(Copy, Clone)]
pub struct Card
{
    ch: [char; 3],
    pub val: usize,
    pub has_been_dealt: bool,
}

impl Card
{
    pub fn new(display: [char; 3], val_new: usize) -> Card
    {
        Card { ch: (display), val: (val_new), has_been_dealt: false }
    }

    pub fn render(&self)
    {
        let ch_string: String = self.ch.iter().collect();
        print!("{ch_string}");
    }

    pub fn render_debug(&self)
    {
        let ch_string: String = self.ch.iter().collect();
        print!("Display: {ch_string}, Value: {0}", self.val);
    }
}

pub struct Deck
{
    pub cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Deck
    {
        let card_displays: [&str; 13] = [
            "A",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "10",
            "J",
            "Q",
            "K"
        ];

        let suits: [&str; 4] = [
            "♥",
            "♣",
            "♦",
            "♠"
        ];

        let mut card_buffer: [Card; 52] = [Card::new([' ', ' ', ' '], 0); 52];

        for i in 0..4
        {
            for j in 0..13
            {
                //Calculates the suit and value of the card, and composes it into a string
                //Adds additional " " to make sure the length is always 3 or more
                let card_display_string: String = card_displays[j].to_string().add(suits[i]).add(" ");

                card_buffer[j + i * 13] = Card::new([
                        card_display_string.chars().nth(0).unwrap(),
                        card_display_string.chars().nth(1).unwrap(),
                        card_display_string.chars().nth(2).unwrap()
                    ],
                    (j + 1).clamp(1, 10));
            }
        }

        return  Deck {
            cards: card_buffer,
        };
    }
}

pub struct Hand
{
    pub cards: Vec<Card>,
    pub value: usize,
}

impl Hand
{
    pub fn new() -> Hand
    {
        Hand {
            cards: Vec::new(),
            value: 0
        }
    }

    pub fn draw_cards(&mut self, amount: usize, deck: &mut Deck)
    {
        for i in 0..amount
        {
            let mut drawn_card: &mut Card = &mut deck.cards[rng().random_range(0..51)];

            while drawn_card.has_been_dealt {
                drawn_card = &mut deck.cards[rng().random_range(0..51)];
            }
    
            drawn_card.has_been_dealt = true;
            self.cards.push(*drawn_card);
        }
    }

    pub fn render_hand(&self, show_full: bool)
    {
        if show_full
        {
            for card in &self.cards {
                card.render();
                print!(" ");
            }
        }else {
            self.cards[0].render();
        }
    }

    pub fn calculate_hand(&mut self)
    {
        let mut has_ace: bool = false;

        self.value = 0;
        for card in &self.cards {
            if card.val == 1
            {
                has_ace = true;
            }
            
            self.value += card.val;
        }

        if(has_ace && self.value <= 11)
        {
            self.value += 10;
        }
    }
}