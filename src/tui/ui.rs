use crate::tui::app::{App, View};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App) {
    match app.current_view {
        View::PhonemeSelection => render_phoneme_selection(frame, app),
        View::Preview => render_preview(frame, app),
    }
}

fn render_phoneme_selection(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // メインレイアウト: タイトル + コンテンツ + ステータスバー
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // タイトル
            Constraint::Min(0),          // コンテンツ
            Constraint::Length(3),       // ステータスバー
        ])
        .split(size);

    // タイトル
    let title = Paragraph::new("Phoneme Reverser TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // 3カラムレイアウト: 母音 | 子音 | 選択済み
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),  // 母音
            Constraint::Percentage(40),  // 子音
            Constraint::Percentage(30),  // 選択済み
        ])
        .split(chunks[1]);

    // 母音リスト
    let vowels: Vec<ListItem> = app
        .phoneme_db
        .get_vowels()
        .iter()
        .map(|p| {
            ListItem::new(format!("[{}] {} ({})", p.key, p.description_ja, p.ipa))
                .style(Style::default().fg(Color::Yellow))
        })
        .collect();

    let vowels_list = List::new(vowels).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Vowels (母音)")
            .style(Style::default().fg(Color::White)),
    );
    frame.render_widget(vowels_list, content_chunks[0]);

    // 子音リスト
    let consonants: Vec<ListItem> = app
        .phoneme_db
        .get_consonants()
        .iter()
        .map(|p| {
            ListItem::new(format!("[{}] {} ({})", p.key, p.description_ja, p.ipa))
                .style(Style::default().fg(Color::Green))
        })
        .collect();

    let consonants_list = List::new(consonants).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Consonants (子音)")
            .style(Style::default().fg(Color::White)),
    );
    frame.render_widget(consonants_list, content_chunks[1]);

    // 選択済み音素リスト
    let selected_items: Vec<String> = app
        .selected_phonemes
        .iter()
        .enumerate()
        .map(|(i, p)| format!("{}. [{}] {}", i + 1, p.ipa, p.description_ja))
        .collect();

    let selected_text = if selected_items.is_empty() {
        vec![
            Line::from("(No phonemes selected)"),
            Line::from(""),
            Line::from(Span::styled(
                "Press letter keys to add",
                Style::default().fg(Color::Gray),
            )),
        ]
    } else {
        let mut lines: Vec<Line> = selected_items.iter().map(|s| Line::from(s.as_str())).collect();
        lines.push(Line::from(""));
        lines.push(Line::from(format!("Total: {}", selected_items.len())));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "[Enter] Preview",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "[Backspace] Delete",
            Style::default().fg(Color::Red),
        )));
        lines
    };

    let selected_list = Paragraph::new(selected_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Selected Phonemes")
            .style(Style::default().fg(Color::White)),
    );
    frame.render_widget(selected_list, content_chunks[2]);

    // ステータスバー
    let language = app.current_language();
    let status_text = format!("Language: {}  |  [l] Switch  |  [q] Quit", language.display_name());
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(status, chunks[2]);
}

fn render_preview(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // メインレイアウト
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // タイトル
            Constraint::Min(0),          // コンテンツ
            Constraint::Length(3),       // ステータスバー
        ])
        .split(size);

    // タイトル
    let title = Paragraph::new("Preview & Playback")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // コンテンツ
    let original: String = app
        .selected_phonemes
        .iter()
        .map(|p| format!("[{}]", p.ipa))
        .collect::<Vec<_>>()
        .join(" ");

    let reversed: String = app
        .get_reversed_phonemes()
        .iter()
        .map(|p| format!("[{}]", p.ipa))
        .collect::<Vec<_>>()
        .join(" ");

    let content_text = vec![
        Line::from(""),
        Line::from(format!("Original:  {}", original)),
        Line::from(format!("Reversed:  {}", reversed)),
        Line::from(""),
        Line::from(Span::styled(
            "[p] Play Original",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "[r] Play Reversed",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "[s] Save to <timestamp>.wav",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("Status: {:?}", app.playback_status),
            Style::default().fg(Color::Gray),
        )),
    ];

    let content = Paragraph::new(content_text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left);
    frame.render_widget(content, chunks[1]);

    // ステータスバー
    let language = app.current_language();
    let status_text = format!("Language: {}  |  [l] Switch  |  [Esc] Back  |  [q] Quit", language.display_name());
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(status, chunks[2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::{Constraint, Rect};

    #[test]
    fn test_layout_calculation() {
        // 画面を3分割するレイアウト
        let area = Rect::new(0, 0, 100, 30);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(area);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].width, 30);
        assert_eq!(chunks[1].width, 40);
        assert_eq!(chunks[2].width, 30);
    }
}
