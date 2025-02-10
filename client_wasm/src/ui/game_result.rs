use super::*;

fn ui_hold_result_right(mut hold: Vec<Card>) {
    if hold.len() == 0 {
        draw_text_oneline_center_color(&RESULT_HOLD_RIGHT_EMPTY, "CLEAR!", CARD_CLEAR_BORDER);
        return;
    }

    // let r = get_canvas_rect().cut_width([
    //     Percent(80.0),
    //     Fixed(CARD_V_WIDTH),
    // ])[1].cut_height([
    //     Percent(25.0),
    //     Fixed(CARD_V_HEIGHT),
    // ])[1].clone();
    // warn!("{:?}", r);

    let hn = hold.len();
    let (h1, h2, h3) = if hn > 10 {
        let h3 = hold.split_off(10);
        let h2 = hold.split_off(5);
        (hold, h2, h3)
    } else if hn > 5 {
        let h2 = hold.split_off(5);
        (hold, h2, vec![])
    } else {
        (hold, vec![], vec![])
    };

    let mut line1 = RESULT_HOLD_RIGHT_START.clone();
    let mut line2 = line1.clone();
    line2.y += line1.h + DESK_MY_HOLD_GAP_HEIGHT;
    let mut line3 = line2.clone();
    line3.y += line2.h + DESK_MY_HOLD_GAP_HEIGHT;

    for c in h1.into_iter().rev() {
        ui_card_vertical(&line1, Some(c), Some(MYCARD_BORDER));
        line1.x -= line1.w + RESULT_HOLD_GAP_WIDTH;
    }
    for c in h2.into_iter().rev() {
        ui_card_vertical(&line2, Some(c), Some(MYCARD_BORDER));
        line2.x -= line2.w + RESULT_HOLD_GAP_WIDTH;
    }
    for c in h3.into_iter().rev() {
        ui_card_vertical(&line3, Some(c), Some(MYCARD_BORDER));
        line3.x -= line3.w + RESULT_HOLD_GAP_WIDTH;
    }
}

fn ui_hold_result_top(hold: Vec<Card>) {
    if hold.len() == 0 {
        draw_text_oneline_center_color(&RESULT_HOLD_TOP_EMPTY, "CLEAR!", CARD_CLEAR_BORDER);
        return;
    }

    // let mut r = get_canvas_rect().cut_width([
    //     Percent(51.0),
    //     Fixed(CARD_V_WIDTH),
    // ])[1].cut_height([
    //     Percent(5.0),
    //     Fixed(CARD_V_HEIGHT),
    // ])[1].clone();
    // warn!("{:?}", r);

    let mut r = RESULT_HOLD_TOP_START.clone();
    for c in hold {
        ui_card_vertical(&r, Some(c), Some(MYCARD_BORDER));
        r.x += r.w + DESK_MY_HOLD_GAP_WIDTH;
    }
}

fn ui_hold_result_left(mut hold: Vec<Card>) {
    if hold.len() == 0 {
        draw_text_oneline_center_color(&RESULT_HOLD_LEFT_EMPTY, "CLEAR!", CARD_CLEAR_BORDER);
        return;
    }

    // let r = get_canvas_rect().cut_width([
    //     Percent(15.0),
    //     Fixed(CARD_V_WIDTH),
    // ])[1].cut_height([
    //     Percent(30.0),
    //     Fixed(CARD_V_HEIGHT),
    // ])[1].clone();
    // warn!("{:?}", r);

    let hn = hold.len();
    let (h1, h2, h3) = if hn > 10 {
        let h3 = hold.split_off(10);
        let h2 = hold.split_off(5);
        (hold, h2, h3)
    } else if hn > 5 {
        let h2 = hold.split_off(5);
        (hold, h2, vec![])
    } else {
        (hold, vec![], vec![])
    };

    let mut line1 = RESULT_HOLD_LEFT_START.clone();
    let mut line2 = line1.clone();
    line2.y += line1.h + DESK_MY_HOLD_GAP_HEIGHT;
    let mut line3 = line2.clone();
    line3.y += line2.h + DESK_MY_HOLD_GAP_HEIGHT;

    for c in h1 {
        ui_card_vertical(&line1, Some(c), Some(MYCARD_BORDER));
        line1.x += line1.w + RESULT_HOLD_GAP_WIDTH;
    }
    for c in h2 {
        ui_card_vertical(&line2, Some(c), Some(MYCARD_BORDER));
        line2.x += line2.w + RESULT_HOLD_GAP_WIDTH;
    }
    for c in h3 {
        ui_card_vertical(&line3, Some(c), Some(MYCARD_BORDER));
        line3.x += line3.w + RESULT_HOLD_GAP_WIDTH;
    }
}

fn ui_hold_result(mut holds: Vec<Vec<Card>>) {
    ui_hold_result_left(holds.pop().unwrap());
    ui_hold_result_top(holds.pop().unwrap());
    ui_hold_result_right(holds.pop().unwrap());
}

fn ui_hold_points(points: Vec<u32>) {
    for (r, p) in RESULT_HOLD_POINTS.into_iter().zip(points).skip(1) {
        draw_text_oneline_center_color(
            &r,
            &format!("POINTS: {}", p),
            HOLD_NUM,
        );
    }
}

pub fn ui_game_result(
    desk: Vec<Vec<Card>>, names: Vec<String>,
    holds: Vec<Vec<Card>>, roomid: String, winner: usize,
) {
    ui_room_id(roomid);

    // continue button
    // let r = get_canvas_rect().cut_width([
    //     Percent(20),
    //     Percent(16),
    // ])[1].cut_height([
    //     Percent(82),
    //     Percent(10),
    // ])[1].clone();
    // warn!("{:?}", r);
    draw_button(&RESULT_CONTINUE_BUTTON, "Continue", true);

    // result msg
    let (msg, color) = if winner == 0 {
        (format!("You win!"), RESULT_MSG_WIN)
    } else {
        let name = &names[winner];
        let name = if get_text_metric(name).0 >= 90.0 {
            &format!("{}..", name.split_at(6).0)
        } else {
            name
        };
        (format!("Player {} wins...", name),
        RESULT_MSG_LOSE)
    };
    draw_text_oneline_center_color(&RESULT_MSG, &msg, color);

    ui_players(names);

    let hold_nums = holds.iter().map(|h| {
        h.len() as u32
    }).collect();
    ui_desk_hold_num(hold_nums);

    let points = holds.iter().map(|h| {
        h.iter().fold(0, |acc, c| acc + c.num)
    }).collect();
    ui_hold_points(points);

    ui_desk_my_holds(holds[0].clone(), holds[0].len() == 0);

    ui_desk(desk);

    ui_hold_result(holds);
}
