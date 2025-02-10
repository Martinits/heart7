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

pub fn get_card_image(c: &Card) -> (String, String){
    let s = match c.suit {
        CardSuit::Spade => "spade".into(),
        CardSuit::Heart => "heart".into(),
        CardSuit::Club => "club".into(),
        CardSuit::Diamond=> "diamond".into(),
    };
    let n = format!("{}_{}", c.num, s);
    (s, n)
}

// If c is None, it means a hold
pub fn ui_card(r: &Rect, c: Option<Card>, border_color: &str) {
    clear_rect(&r);
    draw_rounded_rect_with_r(
        &r,
        8f64,
        border_color
    );
    if let Some(c) = c {
        let (suit, num) = get_card_image(&c);
        draw_image(&Rect{
            x: r.x + 3.0,
            y: r.y + 5.0,
            w: CARD_ICON_WIDTH,
            h: CARD_ICON_HEIGHT,
        }, &num);
        draw_image(&Rect{
            x: r.x + 3.0,
            y: r.y + 23.0,
            w: CARD_ICON_WIDTH,
            h: CARD_ICON_HEIGHT,
        }, &suit);
    } else {
        draw_text_oneline_center(r, "!");
    }
}
