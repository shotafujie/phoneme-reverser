use crate::error::{PhonemeReverserError, Result};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SynthConfig {
    pub language: String,
    pub speed: u32,
    pub pitch: u32,
}

impl Default for SynthConfig {
    fn default() -> Self {
        Self {
            language: "en-us".to_string(),
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
        .arg(&config.language)
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
}
