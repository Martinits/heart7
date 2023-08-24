use crate::tui::color::*;
use crate::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    text::*,
    Frame
};
use crate::game::Card;

pub const NULL_CARD: Card = Card { suit: CardSuit::Spade, num: 1};

pub enum CardAppearance {
    All,
    Vertical,
    Horizontal,
    Empty,
    Hold,
    Half,
    Clear,
    ClearHalf,
}

pub fn get_card_text(card: &Card) -> (String, String) {
    (
        match card.suit {
            CardSuit::Spade => "󰣑", //󱢲
            CardSuit::Heart => "󰣐", //󱢠
            CardSuit::Club => "󰣎", //󱢟
            CardSuit::Diamond => "󰣏", //󱀝
        }.into(),
        match card.num {
            1 => "󰫮".into(),
            2 => "󰬻".into(),
            3 => "󰬼".into(),
            4 => "󰬽".into(),
            5 => "󰬾".into(),
            6 => "󰬿".into(),
            7 => "󰭀".into(),
            8 => "󰭁".into(),
            9 => "󰭂".into(),
            10 => "󰿩".into(),
            11 => "󰫷".into(),
            12 => "󰫾".into(),
            13 => "󰫸".into(),
            _ => panic!("Invalid card num!")
        }
    )
}

pub fn render_card<B: Backend>(
    frame: &mut Frame<B>, card: &Card, a: Rect, ca: CardAppearance,
    dim: bool, highlight: Option<Color>
) {
    let mut block_style = if let Some(c) = highlight {
        Style::default().fg(c)
    } else {
        Style::default().fg(CARD_BORDER)
    };

    if dim {
        block_style = block_style.add_modifier(Modifier::DIM);
    }

    let card_suit_style = match card.suit {
        CardSuit::Spade => Style::default().fg(SPADE),
        CardSuit::Heart => Style::default().fg(HEART),
        CardSuit::Club => Style::default().fg(CLUB),
        CardSuit::Diamond => Style::default().fg(DIAMOND),
    };
    let clear_style = Style::default().fg(CARD_CLEAR);

    let (text_suit, text_num) = get_card_text(card);
    let text = match ca {
        CardAppearance::All => {
            Text::from(
                [
                    Line::styled(text_num.clone(), card_suit_style),
                    Line::styled(
                        format!("{}   {}", text_suit.clone(), text_suit.clone()),
                        card_suit_style
                    ),
                    Line::styled("", card_suit_style),
                    Line::styled("", card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style).alignment(Alignment::Center),
                ].to_vec()
            )
        }
        CardAppearance::Vertical => {
            Text::from(
                [
                    Line::styled(text_num.clone(), card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style),
                ].to_vec()
            )
        }
        CardAppearance::Horizontal => {
            Text::from(
                Line::styled(format!("{} {}", text_num.clone(), text_suit.clone()), card_suit_style),
            )
        }
        CardAppearance::Empty => {
            Text::from(
                [
                    Line::styled("", card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style).alignment(Alignment::Center),
                    Line::styled("", card_suit_style),
                    Line::styled("", card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style).alignment(Alignment::Center),
                ].to_vec()
            )
        }
        CardAppearance::Hold => {
            Text::from(
                [
                    Line::styled("", card_suit_style),
                    Line::styled("HOLD!", card_suit_style).alignment(Alignment::Center),
                    Line::styled("", card_suit_style),
                    Line::styled("", card_suit_style),
                    Line::styled("HOLD!", card_suit_style).alignment(Alignment::Center),
                ].to_vec()
            )
        }
        CardAppearance::Clear => {
            Text::from(
                [
                    Line::styled("", clear_style),
                    Line::styled("CLEAR", clear_style).alignment(Alignment::Center),
                    Line::styled("", clear_style),
                    Line::styled("", clear_style),
                    Line::styled("CLEAR", clear_style).alignment(Alignment::Center),
                ].to_vec()
            )
        }
        CardAppearance::ClearHalf => {
            Text::from(
                [
                    Line::styled("", clear_style),
                    Line::styled("CLEAR", clear_style).alignment(Alignment::Center),
                ].to_vec()
            )
        }
        CardAppearance::Half => {
            Text::from(
                [
                    Line::styled(text_num.clone(), card_suit_style),
                    Line::styled(
                        format!("{}   {}", text_suit.clone(), text_suit.clone()),
                        card_suit_style
                    ),
                ].to_vec()
            )
        }
    };

    // clear first
    frame.render_widget(Clear, a);

    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(block_style)
            ),
        a
    )
}
