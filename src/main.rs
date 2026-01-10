use anyhow::{Context, Result};
use phoneme_reverser::{
    converter::PhonemeConverter,
    phoneme::PhonemeRecognizer,
    synth::{synthesize_phonemes, SynthConfig},
};
use std::path::Path;

fn main() -> Result<()> {
    println!("=== phoneme-reverser MVP ===\n");

    // ハードコードされたファイルパス
    let input_path = Path::new("input.wav");
    let output_path = Path::new("output.wav");

    // 入力ファイルの存在確認
    if !input_path.exists() {
        eprintln!("Error: input.wav not found");
        eprintln!("Please place an audio file as 'input.wav' in the current directory.");
        std::process::exit(1);
    }

    // 1. 音素認識
    println!("[1/4] Initializing phoneme recognizer...");
    let recognizer = PhonemeRecognizer::new()
        .context("Failed to initialize phoneme recognizer")?;

    println!("[2/4] Recognizing phonemes from input.wav...");
    let ipa_phonemes = recognizer.recognize(input_path)
        .context("Failed to recognize phonemes")?;
    println!("  IPA phonemes: {:?}", ipa_phonemes);

    // 2. 逆順
    let reversed: Vec<String> = ipa_phonemes.into_iter().rev().collect();
    println!("\n  Reversed: {:?}", reversed);

    // 3. IPA → espeak変換
    println!("\n[3/4] Converting IPA to espeak notation...");
    let converter = PhonemeConverter::new()
        .context("Failed to initialize phoneme converter")?;

    let espeak_phonemes = converter.convert_ipa_to_espeak(&reversed)
        .context("Failed to convert phonemes")?;
    println!("  eSpeak phonemes: {}", espeak_phonemes);

    // 4. 合成
    println!("\n[4/4] Synthesizing speech...");
    synthesize_phonemes(&espeak_phonemes, output_path, &SynthConfig::default())
        .context("Failed to synthesize phonemes")?;

    println!("\n=== Success! ===");
    println!("Synthesized audio saved to: {}", output_path.display());
    println!("\nPlay the output:");
    println!("  afplay output.wav  # macOS");
    println!("  aplay output.wav   # Linux");

    Ok(())
}
