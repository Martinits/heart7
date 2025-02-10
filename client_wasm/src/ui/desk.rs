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

pub fn ui_desk_hold_num(hold_nums: Vec<u32>) {
    // let r = [
    //     // right
    //     get_canvas_rect().cut_width([
    //         Percent(85),
    //         Percent(13),
    //     ])[1].cut_height([
    //         Percent(43),
    //         Percent(5),
    //     ])[1].clone(),
    //
    //     // top
    //     get_canvas_rect().cut_width([
    //         Percent(42),
    //         Percent(13),
    //     ])[1].cut_height([
    //         Percent(8),
    //         Percent(5),
    //     ])[1].clone(),
    //
    //     // left
    //     get_canvas_rect().cut_width([
    //         Percent(1),
    //         Percent(13),
    //     ])[1].cut_height([
    //         Percent(48),
    //         Percent(5),
    //     ])[1].clone(),
    // ];

    for (r, hn) in DESK_HOLD_NUM.into_iter().zip(hold_nums).skip(1) {
        // warn!("{:?}", r);
        // draw_rect(&r, BORDER_LIGHT);
        draw_text_oneline_center_color(
            &r,
            &format!("HOLD: {}", hn),
            HOLD_NUM,
        );
    }
}

pub fn ui_desk_my_holds(my_holds: Vec<Card>, clear: bool) {

}
