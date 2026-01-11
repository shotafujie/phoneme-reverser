use crate::error::Result;
use std::path::Path;

pub struct AudioData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

pub fn read_wav(path: &Path) -> Result<AudioData> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();

    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .samples::<f32>()
            .collect::<std::result::Result<Vec<_>, _>>()?,
        hound::SampleFormat::Int => reader
            .samples::<i16>()
            .map(|s| s.map(|s| s as f32 / i16::MAX as f32))
            .collect::<std::result::Result<Vec<_>, _>>()?,
    };

    Ok(AudioData {
        samples,
        sample_rate: spec.sample_rate,
        channels: spec.channels,
    })
}

pub fn write_wav(path: &Path, data: &AudioData) -> Result<()> {
    let spec = hound::WavSpec {
        channels: data.channels,
        sample_rate: data.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;

    for &sample in &data.samples {
        let sample_i16 = (sample * i16::MAX as f32) as i16;
        writer.write_sample(sample_i16)?;
    }

    writer.finalize()?;
    Ok(())
}

pub fn play_audio(_data: &AudioData) -> Result<()> {
    // TODO: Implement CPAL integration in Phase 2
    // For MVP, skip audio playback
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_read_wav_valid_file() {
        // tests/fixtures/sample.wavを読み込む
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&manifest_dir).join("tests/fixtures/sample.wav");

        let result = read_wav(&path);
        assert!(result.is_ok(), "Failed to read valid WAV file");

        let data = result.unwrap();
        assert!(data.samples.len() > 0, "Sample data is empty");
        assert!(data.sample_rate > 0, "Sample rate is invalid");
        assert!(data.channels > 0, "Channel count is invalid");
    }

    #[test]
    fn test_read_wav_invalid_file() {
        let path = Path::new("nonexistent.wav");
        let result = read_wav(path);
        assert!(result.is_err(), "Should fail for nonexistent file");
    }

    #[test]
    fn test_write_wav_roundtrip() {
        // オリジナルのデータを作成
        let original = AudioData {
            samples: vec![0.0, 0.5, -0.5, 1.0],
            sample_rate: 16000,
            channels: 1,
        };

        // 一時ファイルに書き込み
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().join("test.wav");

        let write_result = write_wav(&temp_path, &original);
        assert!(write_result.is_ok(), "Failed to write WAV file");

        // 書き込んだファイルを読み込み
        let loaded = read_wav(&temp_path).unwrap();

        // データが一致することを確認
        assert_eq!(loaded.sample_rate, original.sample_rate);
        assert_eq!(loaded.channels, original.channels);
        assert_eq!(loaded.samples.len(), original.samples.len());

        // サンプルデータの比較（浮動小数点の誤差を考慮）
        for (i, (orig, load)) in original.samples.iter().zip(loaded.samples.iter()).enumerate() {
            let diff = (orig - load).abs();
            assert!(
                diff < 0.01,
                "Sample {} differs too much: orig={}, loaded={}, diff={}",
                i,
                orig,
                load,
                diff
            );
        }
    }
}
