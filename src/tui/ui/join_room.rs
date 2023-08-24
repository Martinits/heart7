use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    Frame
};
use tui_input::Input;
use super::common::*;

pub fn join_room<B: Backend>(frame: &mut Frame<B>, input: &Input, msg: &String) {
    let prompt = render_prompt_window(frame);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(prompt);

    frame.render_widget(
        Paragraph::new(msg.as_str())
        .alignment(Alignment::Center)
        .style(Style::default().fg(TEXT_NORMAL).bold()),
        chunks[0],
    );

    let input_rect = rect_cut_center(chunks[1], -3, 60);
    let input_width = input_rect.width.max(3) - 3;
    let scroll = input.visual_scroll(input_width as usize);
    frame.render_widget(
        Paragraph::new(input.value())
            .style(Style::default().fg(INPUT_BORDER))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Room ID")),
        input_rect,
    );
    frame.set_cursor(
        // Put cursor past the end of the input text
        input_rect.x
            + ((input.visual_cursor()).max(scroll) - scroll) as u16
            + 1,
        // Move one line down, from the border to the input line
        input_rect.y + 1,
    );

    let button_rect = rect_cut_center(chunks[2], -3, 20);
    frame.render_widget(get_button("Join Room!", true), button_rect);
}
