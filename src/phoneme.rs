use crate::error::{PhonemeReverserError, Result};
use pyo3::prelude::*;
use std::path::Path;

pub struct PhonemeRecognizer {
    // PyO3のPython GILとモデルを保持
    model: Py<PyAny>,
}

impl PhonemeRecognizer {
    pub fn new() -> Result<Self> {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            // Add venv site-packages to sys.path
            let sys = PyModule::import_bound(py, "sys")
                .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                    "Failed to import sys: {}",
                    e
                )))?;

            // Get current working directory and construct venv path
            let cwd = std::env::current_dir()
                .map_err(|e| PhonemeReverserError::Io(e))?;
            let venv_site_packages = cwd.join(".venv/lib/python3.13/site-packages");

            if venv_site_packages.exists() {
                let path_list = sys.getattr("path")
                    .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                        "Failed to get sys.path: {}",
                        e
                    )))?;
                let venv_path_str = venv_site_packages.to_str().ok_or_else(|| {
                    PhonemeReverserError::PhonemeRecognition("Invalid UTF-8 in venv path".to_string())
                })?;
                path_list.call_method1("insert", (0, venv_path_str))
                    .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                        "Failed to add venv to sys.path: {}",
                        e
                    )))?;
            }

            // allosaurus.appモジュールをインポート
            let allosaurus_app = PyModule::import_bound(py, "allosaurus.app")
                .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                    "Failed to import allosaurus.app: {}",
                    e
                )))?;

            // read_recognizer関数を呼び出してモデルをロード
            let model = allosaurus_app
                .getattr("read_recognizer")
                .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                    "Failed to get read_recognizer: {}",
                    e
                )))?
                .call0()
                .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                    "Failed to call read_recognizer: {}",
                    e
                )))?;

            Ok(Self {
                model: model.unbind(),
            })
        })
    }

    pub fn recognize(&self, wav_path: &Path) -> Result<Vec<String>> {
        Python::with_gil(|py| {
            let model = self.model.bind(py);

            // recognize メソッドを呼び出し
            let wav_path_str = wav_path
                .to_str()
                .ok_or_else(|| PhonemeReverserError::PhonemeRecognition(
                    "Invalid UTF-8 in path".to_string()
                ))?;

            let result = model
                .call_method1("recognize", (wav_path_str,))
                .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                    "Failed to call recognize: {}",
                    e
                )))?;

            // 結果を文字列として抽出
            let phoneme_str: String = result
                .extract()
                .map_err(|e| PhonemeReverserError::PhonemeRecognition(format!(
                    "Failed to extract result: {}",
                    e
                )))?;

            // スペース区切りで音素列をパース
            let phonemes: Vec<String> = phoneme_str
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            Ok(phonemes)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_phoneme_recognizer_initialization() {
        // Check which Python is being used
        Python::with_gil(|py| {
            let sys = py.import_bound("sys").unwrap();
            let executable: String = sys.getattr("executable").unwrap().extract().unwrap();
            eprintln!("Python executable: {}", executable);

            let path: Vec<String> = sys.getattr("path").unwrap().extract().unwrap();
            eprintln!("Python path: {:?}", path);
        });

        let result = PhonemeRecognizer::new();
        if let Err(e) = &result {
            eprintln!("Error: {:?}", e);
        }
        assert!(result.is_ok(), "Failed to initialize PhonemeRecognizer: {:?}", result.err());
    }

    #[test]
    fn test_recognize_phonemes_from_wav() {
        let recognizer = PhonemeRecognizer::new().unwrap();

        // tests/fixtures/sample.wavを使用
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&manifest_dir).join("tests/fixtures/sample.wav");

        let result = recognizer.recognize(&path);
        assert!(result.is_ok(), "Failed to recognize phonemes");

        let phonemes = result.unwrap();
        assert!(phonemes.len() > 0, "No phonemes recognized");

        // 音素が妥当な形式（通常1-4文字）かチェック
        for p in &phonemes {
            assert!(
                p.len() <= 4,
                "Phoneme '{}' is too long (expected <= 4 chars)",
                p
            );
        }

        println!("Recognized phonemes: {:?}", phonemes);
    }
}
