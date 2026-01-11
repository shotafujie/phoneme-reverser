use crate::error::{PhonemeReverserError, Result};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Japanese,
    English,
}

impl Language {
    pub fn to_espeak_code(&self) -> &str {
        match self {
            Language::Japanese => "ja",
            Language::English => "en-us",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Language::Japanese => "日本語",
            Language::English => "English",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Japanese
    }
}

#[derive(Debug, Clone)]
pub struct SynthConfig {
    pub language: Language,
    pub speed: u32,
    pub pitch: u32,
}

impl Default for SynthConfig {
    fn default() -> Self {
        Self {
            language: Language::default(),
            speed: 175,
            pitch: 50,
        }
    }
}

pub fn synthesize_phonemes(
    espeak_phonemes: &str,
    output_path: &Path,
    config: &SynthConfig,
) -> Result<()> {
    // Build espeak-ng command
    // espeak-ng -v <lang> "[[phonemes]]" -w output.wav
    let phoneme_input = format!("[[{}]]", espeak_phonemes);

    let output = Command::new("espeak-ng")
        .arg("-v")
        .arg(config.language.to_espeak_code())
        .arg(&phoneme_input)
        .arg("-w")
        .arg(output_path)
        .output()
        .map_err(|e| PhonemeReverserError::Synthesis(format!(
            "Failed to execute espeak-ng: {}. Is espeak-ng installed?",
            e
        )))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(PhonemeReverserError::Synthesis(format!(
            "espeak-ng failed with exit code {:?}: {}",
            output.status.code(),
            stderr
        )));
    }

    // Verify output file was created
    if !output_path.exists() {
        return Err(PhonemeReverserError::Synthesis(
            "espeak-ng did not create output file".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::read_wav;
    use tempfile::tempdir;

    #[test]
    fn test_language_default_is_japanese() {
        assert_eq!(Language::default(), Language::Japanese);
    }

    #[test]
    fn test_language_to_espeak_code() {
        assert_eq!(Language::Japanese.to_espeak_code(), "ja");
        assert_eq!(Language::English.to_espeak_code(), "en-us");
    }

    #[test]
    fn test_language_display_name() {
        assert_eq!(Language::Japanese.display_name(), "日本語");
        assert_eq!(Language::English.display_name(), "English");
    }

    #[test]
    fn test_synth_config_default_is_japanese() {
        let config = SynthConfig::default();
        assert_eq!(config.language, Language::Japanese);
    }

    #[test]
    fn test_synthesize_simple_phonemes() {
        // Test basic espeak phoneme synthesis
        let espeak_phonemes = "h @ l oU";  // "hello" in espeak notation
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().join("synth_test.wav");

        let result = synthesize_phonemes(espeak_phonemes, &temp_path, &SynthConfig::default());

        if let Err(e) = &result {
            eprintln!("Synthesis error: {:?}", e);
        }

        assert!(result.is_ok(), "Failed to synthesize phonemes");
        assert!(temp_path.exists(), "Output WAV file was not created");

        // Verify the generated WAV is valid
        let audio = read_wav(&temp_path).unwrap();
        assert!(audio.samples.len() > 0, "Generated WAV has no samples");

        println!("Synthesized {} samples", audio.samples.len());
    }

    #[test]
    fn test_synthesize_with_japanese() {
        // Test Japanese synthesis
        let espeak_phonemes = "a i u e o";  // Japanese vowels
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().join("synth_ja_test.wav");

        let mut config = SynthConfig::default();
        config.language = Language::Japanese;

        let result = synthesize_phonemes(espeak_phonemes, &temp_path, &config);

        if let Err(e) = &result {
            eprintln!("Synthesis error: {:?}", e);
        }

        assert!(result.is_ok(), "Failed to synthesize Japanese phonemes");
        assert!(temp_path.exists(), "Output WAV file was not created");

        let audio = read_wav(&temp_path).unwrap();
        assert!(audio.samples.len() > 0, "Generated WAV has no samples");
    }

    #[test]
    fn test_synthesize_with_english() {
        // Test English synthesis
        let espeak_phonemes = "h @ l oU";
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().join("synth_en_test.wav");

        let mut config = SynthConfig::default();
        config.language = Language::English;

        let result = synthesize_phonemes(espeak_phonemes, &temp_path, &config);

        if let Err(e) = &result {
            eprintln!("Synthesis error: {:?}", e);
        }

        assert!(result.is_ok(), "Failed to synthesize English phonemes");
        assert!(temp_path.exists(), "Output WAV file was not created");

        let audio = read_wav(&temp_path).unwrap();
        assert!(audio.samples.len() > 0, "Generated WAV has no samples");
    }
}
