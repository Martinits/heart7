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

fn get_card_image(c: &Card) -> (String, String){
    let s = match c.suit {
        CardSuit::Spade => "spade".into(),
        CardSuit::Heart => "heart".into(),
        CardSuit::Club => "club".into(),
        CardSuit::Diamond=> "diamond".into(),
    };
    let n = format!("{}_{}", c.num, s);
    (s, n)
}

pub fn ui_card(r: &Rect, c: &Card, border_color: &str) {
    clear_rect(&r);
    draw_rounded_rect_with_r(
        &r,
        8f64,
        border_color
    );
    let (suit, num) = get_card_image(&c);
    draw_image(&Rect{
        x: r.x + 3.0,
        y: r.y + 5.0,
        w: 15.0,
        h: 15.0,
    }, &num);
    draw_image(&Rect{
        x: r.x + 3.0,
        y: r.y + 23.0,
        w: 15.0,
        h: 15.0,
    }, &suit);
}
