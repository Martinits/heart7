use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    Frame
};
use super::players::*;
use super::common::*;

pub fn wait_player<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, bool)>,
    msg: &String, roomid: &String)
{
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        vec![false; 4], None
    );

    render_center_msg(frame, msg.clone());

    render_game_info(frame, roomid.clone());

    render_ready_button(frame, false);
}

pub fn wait_ready<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, bool)>,
    msg: &String, roomid: &String)
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

fn render_center_msg<B: Backend>(frame: &mut Frame<B>, msg: String) {
    frame.render_widget(
        Paragraph::new(msg)
            .style(Style::default().fg(CENTER_MSG).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center),
        rect_cut_center(frame.size(), -1, 50)
    )
}
