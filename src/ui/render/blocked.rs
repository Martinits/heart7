use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    text::*,
    widgets::*,
    Frame
};
use super::*;

pub fn blocked<B: Backend>(frame: &mut Frame<B>, sz: (u16, u16)) {
    let a = rect_cut_center(frame.size(), -5, -30);

    let style = Style::default().bold().fg(Color::White);
    let text = Text::from([
        Line::styled("Terminal size too small:", style.clone()),
        Line::from(
            [
                Span::styled("Width = ", style.clone()),
                Span::styled(sz.0.to_string(),
                    if sz.0 < 160 {
                        style.clone().fg(Color::Red)
                    } else {
                        style.clone().fg(Color::Green)
                    }
                ),
                Span::styled(" Height = ", style.clone()),
                Span::styled(sz.1.to_string(),
                    if sz.1 < 48 {
                        style.clone().fg(Color::Red)
                    } else {
                        style.clone().fg(Color::Green)
                    }
                ),
            ].to_vec()
        ),
        Line::default(),
        Line::styled("Minimal size required:", style.clone()),
        Line::styled("Width = 160 Height = 48", style.clone()),
    ].to_vec());

    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center),
        a
    );
}
