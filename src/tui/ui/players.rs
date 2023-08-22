use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    text::*,
    Frame
};
use super::common::*;

fn render_ready<B: Backend>(frame: &mut Frame<B>, a: Rect) {
    frame.render_widget(
        Paragraph::new("READY!")
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(READY)
                    .add_modifier(Modifier::BOLD)
            ),
        a
    )
}
pub fn render_players<B: Backend>(frame: &mut Frame<B>, names: &Vec<String>,
    ready: Vec<bool>, holds: Option<Vec<u32>>
) {
    // myself
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(11),
            ].as_ref()
        )
        .split(frame.size())[1];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Length(14),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[1];
    render_one_player(frame, names[0].clone(), a, None);

    // right one
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Length(11),
                Constraint::Min(1)
            ].as_ref()
        )
        .split(frame.size())[1];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(14),
            ].as_ref()
        )
        .split(a)[1];
    render_one_player(frame, names[1].clone(), a, None);

    // top one
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Length(11),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(frame.size())[0];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Length(14),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[1];
    render_one_player(frame, names[2].clone(), a, None);

    // left one
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Length(11),
                Constraint::Min(1)
            ].as_ref()
        )
        .split(frame.size())[1];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Length(14),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[0];
    render_one_player(frame, names[3].clone(), a, None);

    // ready
    // myself
    if ready[0] {
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(11),
                ].as_ref()
            )
            .split(frame.size())[1];
        a = rect_cut_center(a, -3, 20);
        render_ready(frame, a);
    }

    // right one
    if ready[1] {
        let mut a = rect_cut_center(frame.size(), -3, 100);
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Percentage(10),
                    Constraint::Length(20),
                ].as_ref()
            )
            .split(a)[1];
        render_ready(frame, a);
    }

    // top one
    if ready[2] {
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Length(13),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(frame.size())[0];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(10),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[1];
        a = rect_cut_center(a, -3, 100);
        render_ready(frame, a);
    }

    // left one
    if ready[3] {
        let mut a = rect_cut_center(frame.size(), -3, 100);
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Length(20),
                    Constraint::Percentage(10),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[1];
        render_ready(frame, a);
    }

    // hold num
    if let Some(holds) = holds {
        // right
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Length(13),
                    Constraint::Length(3),
                    Constraint::Min(1)
                ].as_ref()
            )
            .split(frame.size())[2];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(16),
                ].as_ref()
            )
            .split(a)[1];
        render_hold_num(frame, a, holds[1]);

        // top
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Length(13),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(frame.size())[0];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(10),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[1];
        a = rect_cut_center(a, -3, 100);
        render_hold_num(frame, a, holds[2]);

        // left
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Length(13),
                    Constraint::Length(3),
                    Constraint::Min(1)
                ].as_ref()
            )
            .split(frame.size())[2];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Length(12),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[0];
        render_hold_num(frame, a, holds[3]);
    }
}

pub fn render_one_player<B:Backend>(
    frame: &mut Frame<B>, name: String, a: Rect, border_color: Option<Color>
) {
    // card sign
    let (width, height) = (a.width - 2, a.height - 2);
    let blocks = [
        Rect::new(a.x + 2, a.y,     width, height),
        Rect::new(a.x + 1, a.y + 1, width, height),
        Rect::new(a.x    , a.y + 2, width, height),
    ];
    for b in blocks.into_iter() {
        frame.render_widget(Clear, b);
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(
                    if let Some(c) = border_color {
                        c
                    } else {
                        CARD_SIGN
                })),
            b
        );
    }

    // name
    frame.render_widget(
        Paragraph::new(
            Text::from(
                if name == "" {
                    Span::styled(
                        "???",
                        Style::default().bold().fg(NAME_DIM).add_modifier(Modifier::DIM)
                    )
                } else {
                    Span::styled(
                        name,
                        Style::default().bold().fg(NAME)
                    )
                }
            )
        )
        .alignment(Alignment::Center),
        rect_cut_center(blocks[2].inner(&Margin{vertical: 1, horizontal: 1}), -1, 100)
    )
}

fn render_hold_num<B: Backend>(frame: &mut Frame<B>, a: Rect, num: u32) {
    frame.render_widget(
        Paragraph::new(format!("HOLD: {}", num))
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(HOLD_NUM)
                    .add_modifier(Modifier::BOLD)
            ),
        a
    )
}

