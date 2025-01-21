use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    Frame
};
use tui_input::Input;
use super::*;

pub fn ui_home_page<B: Backend>(frame: &mut Frame<B>, input: &Input, msg: &String, connecting: &bool) {
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
            .style(Style::default().fg(
                if *connecting {
                    INPUT_BORDER_BLOCK
                } else {
                    INPUT_BORDER
                }
            ))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("IP:PORT")),
        input_rect,
    );
    if !connecting {
        frame.set_cursor(
            // Put cursor past the end of the input text
            input_rect.x
                + ((input.visual_cursor()).max(scroll) - scroll) as u16
                + 1,
            // Move one line down, from the border to the input line
            input_rect.y + 1,
        );
    }

    let button_rect = rect_cut_center(chunks[2], -3, 20);
    frame.render_widget(get_button("GO!", !connecting), button_rect);
}
