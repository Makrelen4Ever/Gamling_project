use std::ops::{Add};

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
