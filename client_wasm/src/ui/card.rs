use super::*;

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
pub fn ui_card_vertical(r: &Rect, c: Option<Card>, border_color: Option<&str>) {
    clear_rect(&r);
    draw_rounded_rect_with_r(
        &r,
        8f64,
        border_color.unwrap_or(CARD_BORDER),
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
            y: r.y + 22.0,
            w: CARD_ICON_WIDTH,
            h: CARD_ICON_HEIGHT,
        }, &suit);
    } else {
        draw_text_oneline_center(r, "!");
    }
}

// If c is None, it means a empty card, need to draw as ???
pub fn ui_card_horizontal(r: &Rect, c: Option<Card>, border_only: bool) {
    clear_rect(&r);
    draw_rounded_rect_with_r(
        &r,
        8f64,
        CARD_BORDER,
    );

    if border_only {
        return;
    }

    if let Some(c) = c {
        let (suit, num) = get_card_image(&c);
        draw_image(&Rect{
            x: r.x + 3.0,
            y: r.y + 4.0,
            w: CARD_ICON_WIDTH,
            h: CARD_ICON_HEIGHT,
        }, &num);
        draw_image(&Rect{
            x: r.x + 21.0,
            y: r.y + 4.0,
            w: CARD_ICON_WIDTH,
            h: CARD_ICON_HEIGHT,
        }, &suit);
    } else {
        draw_text_oneline_center(r, "???");
    }
}
