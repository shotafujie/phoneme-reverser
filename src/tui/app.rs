use crate::converter::PhonemeConverter;
use crate::error::Result;
use crate::playback::AudioPlayer;
use crate::synth::{synthesize_phonemes, SynthConfig};
use crate::tui::phoneme_db::{Phoneme, PhonemeDatabase};
use chrono::Local;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    PhonemeSelection,
    Preview,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackStatus {
    Idle,
    Synthesizing,
    Playing,
    Error(String),
}

pub struct App {
    // UI State
    pub should_quit: bool,
    pub current_view: View,

    // Phoneme Data
    pub selected_phonemes: Vec<Phoneme>,
    pub phoneme_db: PhonemeDatabase,

    // Audio State
    pub is_playing: bool,
    pub playback_status: PlaybackStatus,

    // Backend services
    converter: PhonemeConverter,
    synth_config: SynthConfig,
    player: AudioPlayer,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            current_view: View::PhonemeSelection,
            selected_phonemes: Vec::new(),
            phoneme_db: PhonemeDatabase::new(),
            is_playing: false,
            playback_status: PlaybackStatus::Idle,
            converter: PhonemeConverter::new()?,
            synth_config: SynthConfig::default(),
            player: AudioPlayer::new()?,
        })
    }

    pub fn select_phoneme(&mut self, key: char) {
        if let Some(phoneme) = self.phoneme_db.get_by_key(key) {
            self.selected_phonemes.push(phoneme.clone());
        }
    }

    pub fn delete_last_phoneme(&mut self) {
        self.selected_phonemes.pop();
    }

    pub fn get_reversed_phonemes(&self) -> Vec<Phoneme> {
        self.selected_phonemes.iter().rev().cloned().collect()
    }

    pub fn toggle_view(&mut self) {
        self.current_view = match self.current_view {
            View::PhonemeSelection => View::Preview,
            View::Preview => View::PhonemeSelection,
        };
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn play_original(&mut self) -> Result<()> {
        if self.selected_phonemes.is_empty() {
            self.playback_status = PlaybackStatus::Error("No phonemes selected".to_string());
            return Ok(());
        }

        self.playback_status = PlaybackStatus::Synthesizing;

        // 音素列をIPA文字列のVecに変換
        let ipa_phonemes: Vec<String> = self.selected_phonemes.iter().map(|p| p.ipa.clone()).collect();

        // IPA → eSpeak変換
        let espeak_phonemes = self.converter.convert_ipa_to_espeak(&ipa_phonemes)?;

        // 一時ファイルに合成
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path();

        synthesize_phonemes(&espeak_phonemes, temp_path, &self.synth_config)?;

        // 音声を読み込んで再生
        self.playback_status = PlaybackStatus::Playing;
        let audio = crate::audio::read_wav(temp_path)?;
        self.player.play(&audio)?;

        self.playback_status = PlaybackStatus::Idle;
        Ok(())
    }

    pub fn play_reversed(&mut self) -> Result<()> {
        if self.selected_phonemes.is_empty() {
            self.playback_status = PlaybackStatus::Error("No phonemes selected".to_string());
            return Ok(());
        }

        self.playback_status = PlaybackStatus::Synthesizing;

        // 逆順音素列をIPA文字列のVecに変換
        let reversed_phonemes = self.get_reversed_phonemes();
        let ipa_phonemes: Vec<String> = reversed_phonemes.iter().map(|p| p.ipa.clone()).collect();

        // IPA → eSpeak変換
        let espeak_phonemes = self.converter.convert_ipa_to_espeak(&ipa_phonemes)?;

        // 一時ファイルに合成
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path();

        synthesize_phonemes(&espeak_phonemes, temp_path, &self.synth_config)?;

        // 音声を読み込んで再生
        self.playback_status = PlaybackStatus::Playing;
        let audio = crate::audio::read_wav(temp_path)?;
        self.player.play(&audio)?;

        self.playback_status = PlaybackStatus::Idle;
        Ok(())
    }

    pub fn save_reversed(&mut self) -> Result<String> {
        if self.selected_phonemes.is_empty() {
            self.playback_status = PlaybackStatus::Error("No phonemes selected".to_string());
            return Err(crate::error::PhonemeReverserError::Synthesis(
                "No phonemes to save".to_string(),
            )
            .into());
        }

        self.playback_status = PlaybackStatus::Synthesizing;

        // 逆順音素列をIPA文字列のVecに変換
        let reversed_phonemes = self.get_reversed_phonemes();
        let ipa_phonemes: Vec<String> = reversed_phonemes.iter().map(|p| p.ipa.clone()).collect();

        // IPA → eSpeak変換
        let espeak_phonemes = self.converter.convert_ipa_to_espeak(&ipa_phonemes)?;

        // wav/ディレクトリを作成（存在しない場合）
        let wav_dir = PathBuf::from("wav");
        std::fs::create_dir_all(&wav_dir)?;

        // タイムスタンプ形式のファイル名を生成
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let filename = format!("{}.wav", timestamp);
        let output_path = wav_dir.join(&filename);

        // 音声合成
        synthesize_phonemes(&espeak_phonemes, &output_path, &self.synth_config)?;

        self.playback_status = PlaybackStatus::Idle;
        Ok(format!("wav/{}", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let app = App::new().unwrap();
        assert_eq!(app.current_view, View::PhonemeSelection);
        assert_eq!(app.selected_phonemes.len(), 0);
        assert!(!app.should_quit);
        assert!(!app.is_playing);
        assert_eq!(app.playback_status, PlaybackStatus::Idle);
    }

    #[test]
    fn test_select_phoneme() {
        let mut app = App::new().unwrap();
        app.select_phoneme('a');

        assert_eq!(app.selected_phonemes.len(), 1);
        assert_eq!(app.selected_phonemes[0].ipa, "a");
    }

    #[test]
    fn test_select_multiple_phonemes() {
        let mut app = App::new().unwrap();
        app.select_phoneme('a');
        app.select_phoneme('k');
        app.select_phoneme('a');

        assert_eq!(app.selected_phonemes.len(), 3);
        assert_eq!(app.selected_phonemes[0].ipa, "a");
        assert_eq!(app.selected_phonemes[1].ipa, "k");
        assert_eq!(app.selected_phonemes[2].ipa, "a");
    }

    #[test]
    fn test_select_invalid_phoneme() {
        let mut app = App::new().unwrap();
        app.select_phoneme('X'); // 無効なキー

        assert_eq!(app.selected_phonemes.len(), 0);
    }

    #[test]
    fn test_delete_phoneme() {
        let mut app = App::new().unwrap();
        app.select_phoneme('a');
        app.select_phoneme('k');
        app.delete_last_phoneme();

        assert_eq!(app.selected_phonemes.len(), 1);
        assert_eq!(app.selected_phonemes[0].ipa, "a");
    }

    #[test]
    fn test_delete_from_empty() {
        let mut app = App::new().unwrap();
        app.delete_last_phoneme(); // 空の状態で削除

        assert_eq!(app.selected_phonemes.len(), 0);
    }

    #[test]
    fn test_reverse_phonemes() {
        let mut app = App::new().unwrap();
        app.select_phoneme('a');
        app.select_phoneme('k');
        app.select_phoneme('u');

        let reversed = app.get_reversed_phonemes();
        assert_eq!(reversed.len(), 3);
        assert_eq!(reversed[0].ipa, "u");
        assert_eq!(reversed[1].ipa, "k");
        assert_eq!(reversed[2].ipa, "a");
    }

    #[test]
    fn test_toggle_view() {
        let mut app = App::new().unwrap();
        assert_eq!(app.current_view, View::PhonemeSelection);

        app.toggle_view();
        assert_eq!(app.current_view, View::Preview);

        app.toggle_view();
        assert_eq!(app.current_view, View::PhonemeSelection);
    }

    #[test]
    fn test_quit() {
        let mut app = App::new().unwrap();
        assert!(!app.should_quit);

        app.quit();
        assert!(app.should_quit);
    }
}
