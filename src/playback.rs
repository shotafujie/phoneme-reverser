use crate::audio::AudioData;
use crate::error::{PhonemeReverserError, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    // プレイヤーは状態を持たない（シンプルな設計）
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn play(&self, audio: &AudioData) -> Result<()> {
        // 1. デフォルトホストとデバイスを取得
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| PhonemeReverserError::AudioPlayback("No output device available".to_string()))?;

        // 2. サポートされている設定を取得
        let config = device
            .default_output_config()
            .map_err(|e| PhonemeReverserError::AudioPlayback(format!("Failed to get default config: {}", e)))?;

        // 3. サンプルデータをクローン
        let samples = audio.samples.clone();
        let channels = audio.channels;
        let total_samples = samples.len();

        // サンプルインデックスを共有（Mutexで保護）
        let sample_idx = Arc::new(Mutex::new(0usize));
        let sample_idx_clone = sample_idx.clone();

        // 4. 出力ストリームのコールバック
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                let config: cpal::StreamConfig = config.into();
                device
                    .build_output_stream(
                        &config,
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            let mut idx = sample_idx_clone.lock().unwrap();
                            for frame in data.chunks_mut(channels as usize) {
                                if *idx < samples.len() {
                                    for sample in frame.iter_mut() {
                                        *sample = samples[*idx];
                                        *idx += 1;
                                        if *idx >= samples.len() {
                                            break;
                                        }
                                    }
                                } else {
                                    // 再生完了: 無音を出力
                                    for sample in frame.iter_mut() {
                                        *sample = 0.0;
                                    }
                                }
                            }
                        },
                        |err| eprintln!("Stream error: {}", err),
                        None,
                    )
                    .map_err(|e| PhonemeReverserError::AudioPlayback(format!("Failed to build stream: {}", e)))?
            }
            _ => {
                return Err(PhonemeReverserError::AudioPlayback(
                    "Unsupported sample format (only F32 is supported)".to_string(),
                ))
            }
        };

        // 5. ストリームを再生
        stream
            .play()
            .map_err(|e| PhonemeReverserError::AudioPlayback(format!("Failed to play stream: {}", e)))?;

        // 6. 再生完了まで待機
        let duration_secs = total_samples as f32 / (audio.sample_rate * channels as u32) as f32;
        std::thread::sleep(std::time::Duration::from_secs_f32(duration_secs + 0.5));

        Ok(())
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_player_creation() {
        let player = AudioPlayer::new();
        assert!(player.is_ok(), "Failed to create AudioPlayer");
    }

    #[test]
    fn test_play_simple_audio() {
        let player = AudioPlayer::new().unwrap();

        // 440Hz サイン波（ラ音）を1秒生成
        let sample_rate = 44100;
        let duration = 1.0;
        let frequency = 440.0;
        let samples: Vec<f32> = (0..(sample_rate as f32 * duration) as usize)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.5
            })
            .collect();

        let audio = AudioData {
            samples,
            sample_rate,
            channels: 1,
        };

        let result = player.play(&audio);
        assert!(result.is_ok(), "Failed to play audio: {:?}", result.err());
    }

    #[test]
    fn test_play_stereo_audio() {
        let player = AudioPlayer::new().unwrap();

        // ステレオ音声: 左チャンネル440Hz、右チャンネル880Hz
        let sample_rate = 44100;
        let duration = 0.5;
        let mut samples = Vec::new();

        for i in 0..(sample_rate as f32 * duration) as usize {
            let t = i as f32 / sample_rate as f32;
            let left = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.3;
            let right = (2.0 * std::f32::consts::PI * 880.0 * t).sin() * 0.3;
            samples.push(left);
            samples.push(right);
        }

        let audio = AudioData {
            samples,
            sample_rate,
            channels: 2,
        };

        let result = player.play(&audio);
        assert!(result.is_ok(), "Failed to play stereo audio: {:?}", result.err());
    }
}
