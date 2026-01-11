use crate::tui::app::{App, View};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            handle_key_event(app, key);
        }
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.current_view {
        View::PhonemeSelection => handle_phoneme_selection_keys(app, key),
        View::Preview => handle_preview_keys(app, key),
    }
}

fn handle_phoneme_selection_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('l') => app.toggle_language(),
        KeyCode::Char(c) => app.select_phoneme(c),
        KeyCode::Backspace => app.delete_last_phoneme(),
        KeyCode::Enter => {
            if !app.selected_phonemes.is_empty() {
                app.toggle_view();
            }
        }
        _ => {}
    }
}

fn handle_preview_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('l') => app.toggle_language(),
        KeyCode::Esc => app.toggle_view(),
        KeyCode::Char('p') => {
            if let Err(e) = app.play_original() {
                app.playback_status = crate::tui::app::PlaybackStatus::Error(e.to_string());
            }
        }
        KeyCode::Char('r') => {
            if let Err(e) = app.play_reversed() {
                app.playback_status = crate::tui::app::PlaybackStatus::Error(e.to_string());
            }
        }
        KeyCode::Char('s') => {
            match app.save_reversed() {
                Ok(_filename) => {
                    app.playback_status = crate::tui::app::PlaybackStatus::Idle;
                    // ファイル名を状態に保存したい場合はここで実装
                }
                Err(e) => {
                    app.playback_status = crate::tui::app::PlaybackStatus::Error(e.to_string());
                }
            }
        }
        _ => {}
    }
}
