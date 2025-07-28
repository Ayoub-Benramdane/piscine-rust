use rand::random;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Nb(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let nb: u8 = (random::<u8>() % 4) + 1;
        Suit::translate(nb)
    }

    pub fn translate(value: u8) -> Suit {
        return match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(),
        };
    }
}

impl Rank {
    pub fn random() -> Rank {
        let nb: u8 = (random::<u8>() % 13) + 1;
        Rank::translate(nb)
    }

    pub fn translate(value: u8) -> Rank {
        return match value {
            1 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            2..=10 => Rank::Nb(value),
            _ => unreachable!(),
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card == &Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}
