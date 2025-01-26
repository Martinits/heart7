use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    text::*,
    Frame
};
use super::players::*;
use super::*;

pub fn ui_wait_player<B: Backend>(
    frame: &mut Frame<B>, players: Vec<(String, usize, bool)>,
    msg: Vec<String>, roomid: String)
{
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        vec![false; 4], None
    );

    render_center_msg(frame, msg.clone());

    render_game_info(frame, roomid.clone());

    render_ready_button(frame, false);
}

pub fn ui_wait_ready<B: Backend>(
    frame: &mut Frame<B>, players: Vec<(String, usize, bool)>,
    msg: Vec<String>, roomid: String)
{
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        players.iter().map(|p| p.2).collect::<Vec<bool>>(), None
    );

    render_center_msg(frame, msg.clone());

    render_game_info(frame, roomid.clone());

    if !players[0].2 {
        render_ready_button(frame, true);
    }
}

fn render_ready_button<B: Backend>(frame: &mut Frame<B>, active: bool) {
    let mut button = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(11),
            ].as_ref()
        )
        .split(frame.size())[1];
    button = rect_cut_center(button, -3, 20);

    frame.render_widget(get_button("Get Ready!", active), button);
}

fn render_center_msg<B: Backend>(frame: &mut Frame<B>, msg: Vec<String>) {
    let msg: Vec<Line> = msg.into_iter().map(
        |m| Line::styled(m, Style::default().fg(CENTER_MSG).bold())
    ).collect();
    let lines = msg.len() as i16;

    frame.render_widget(
        Paragraph::new(Text::from(msg))
            .alignment(Alignment::Center),
        rect_cut_center(frame.size(), -lines, 50)
    )
}
