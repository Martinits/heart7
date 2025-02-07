use super::*;

pub const NULL_CARD: Card = Card { suit: CardSuit::Spade, num: 1};

pub enum CardStyle {
    All,
    Vertical,
    Horizontal,
    Empty,
    Hold,
    Half,
    Clear,
    ClearHalf,
}
