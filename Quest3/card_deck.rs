#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Numv(u8),
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
        match rand::random::<u8>() % 4 + 1 {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            4 => Suit::Spade,
            _ => unreachable!(),
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            4 => Suit::Spade,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let n = rand::random::<u8>() % 13 + 1;
        match n {
            1 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            2..=10 => Rank::Numv(n),
            _ => unreachable!(),
        }
    }
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Numv(value),
            _ => unreachable!(),
        }
    }
}

pub fn winner_card(card: &Card) -> bool {
    return card.suit == Suit::Spade && card.rank == Rank::Ace;
}
