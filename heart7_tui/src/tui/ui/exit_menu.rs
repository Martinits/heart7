use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    Frame
};
use super::*;

pub fn render_exit_menu<B: Backend>(frame: &mut Frame<B>, button_num: u32, which: u32) {
    assert!(which < button_num);

    let menu = rect_cut_center(frame.size(), 50, 30);
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(BORDER_LIGHT)),
        menu
    );

    let need = button_num as i16 * 5 - 2;
    let buttons = rect_cut_center(menu, -need, 50);
    let mut constraints = Vec::new();
    for _ in 0..button_num {
        constraints.push(Constraint::Length(3));
        constraints.push(Constraint::Length(2));
    }
    constraints.pop();
    let buttons = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(buttons);

    match button_num {
        2 => {
            frame.render_widget(get_button("Back", which == 0), buttons[0]);
            frame.render_widget(get_button("Exit Program", which == 1), buttons[2]);
        }
        3 => {
            frame.render_widget(get_button("Back", which == 0), buttons[0]);
            frame.render_widget(get_button("Exit Room", which == 1), buttons[2]);
            frame.render_widget(get_button("Exit Program", which == 2), buttons[4]);
        }
        4 => {
            frame.render_widget(get_button("Back", which == 0), buttons[0]);
            frame.render_widget(get_button("Exit Game", which == 1), buttons[2]);
            frame.render_widget(get_button("Exit Room", which == 2), buttons[4]);
            frame.render_widget(get_button("Exit Program", which == 3), buttons[6]);
        }
        _ => panic!("Invalid buttom nums!"),
    }
}
