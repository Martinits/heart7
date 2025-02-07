use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CardStyleOnDesk {
    Normal,
    ThisRound,
    ThisRoundMy,
}

impl Into<Option<&str>> for CardStyleOnDesk {
    fn into(self) -> Option<&'static str> {
        match self {
            CardStyleOnDesk::Normal => None,
            CardStyleOnDesk::ThisRound => Some(CARD_HIGHLIGHT),
            CardStyleOnDesk::ThisRoundMy => Some(CARD_HIGHLIGHT_MY),
        }
    }
}
