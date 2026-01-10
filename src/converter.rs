use crate::error::{PhonemeReverserError, Result};
use pyo3::prelude::*;

pub struct PhonemeConverter;

impl PhonemeConverter {
    pub fn new() -> Result<Self> {
        // PhonemeConverterはstateless（状態を持たない）なので、簡単に初期化
        pyo3::prepare_freethreaded_python();

        // lexconvertが利用可能か確認
        Python::with_gil(|py| {
            // Add venv site-packages to sys.path (same as phoneme.rs)
            let sys = PyModule::import_bound(py, "sys")
                .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                    "Failed to import sys: {}",
                    e
                )))?;

            let cwd = std::env::current_dir()
                .map_err(|e| PhonemeReverserError::Io(e))?;
            let venv_site_packages = cwd.join(".venv/lib/python3.13/site-packages");

            if venv_site_packages.exists() {
                let path_list = sys.getattr("path")
                    .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                        "Failed to get sys.path: {}",
                        e
                    )))?;
                let venv_path_str = venv_site_packages.to_str().ok_or_else(|| {
                    PhonemeReverserError::PhonemeConversion("Invalid UTF-8 in venv path".to_string())
                })?;
                path_list.call_method1("insert", (0, venv_path_str))
                    .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                        "Failed to add venv to sys.path: {}",
                        e
                    )))?;
            }

            // Try to import lexconvert to verify it's available
            PyModule::import_bound(py, "lexconvert")
                .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                    "Failed to import lexconvert: {}",
                    e
                )))?;

            Ok(Self)
        })
    }

    pub fn convert_ipa_to_espeak(&self, ipa_phonemes: &[String]) -> Result<String> {
        Python::with_gil(|py| {
            // Join IPA phonemes with spaces
            let ipa_str = ipa_phonemes.join(" ");

            // Use subprocess to call: .venv/bin/lexconvert --phones2phones unicode-ipa espeak <phonemes>
            let subprocess = PyModule::import_bound(py, "subprocess")
                .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                    "Failed to import subprocess: {}",
                    e
                )))?;

            // Get lexconvert path from venv
            let cwd = std::env::current_dir()
                .map_err(|e| PhonemeReverserError::Io(e))?;
            let lexconvert_path = cwd.join(".venv/bin/lexconvert");

            // Build command: .venv/bin/lexconvert --phones2phones unicode-ipa espeak <ipa_str>
            let mut cmd = vec![
                lexconvert_path.to_str().ok_or_else(|| {
                    PhonemeReverserError::PhonemeConversion("Invalid UTF-8 in lexconvert path".to_string())
                })?.to_string(),
                "--phones2phones".to_string(),
                "unicode-ipa".to_string(),
                "espeak".to_string(),
            ];
            cmd.push(ipa_str.clone());

            // Call subprocess.run()
            let kwargs = pyo3::types::PyDict::new_bound(py);
            kwargs.set_item("capture_output", true)?;
            kwargs.set_item("text", true)?;

            let result = subprocess
                .call_method("run", (cmd,), Some(&kwargs))
                .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                    "Failed to run lexconvert: {}",
                    e
                )))?;

            // Check return code
            let returncode: i32 = result.getattr("returncode")?.extract()?;
            if returncode != 0 {
                let stderr: String = result.getattr("stderr")?.extract()?;
                return Err(PhonemeReverserError::PhonemeConversion(format!(
                    "lexconvert failed with code {}: {}",
                    returncode, stderr
                )));
            }

            // Extract stdout
            let stdout: String = result
                .getattr("stdout")
                .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                    "Failed to get stdout: {}",
                    e
                )))?
                .extract()
                .map_err(|e| PhonemeReverserError::PhonemeConversion(format!(
                    "Failed to extract stdout: {}",
                    e
                )))?;

            Ok(stdout.trim().to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_initialization() {
        let result = PhonemeConverter::new();
        assert!(result.is_ok(), "Failed to initialize PhonemeConverter");
    }

    #[test]
    fn test_convert_ipa_to_espeak() {
        let converter = PhonemeConverter::new().unwrap();

        // Test basic IPA to espeak conversion
        // Note: lexconvert may not support all IPA symbols
        let ipa = vec!["h".to_string(), "ə".to_string(), "l".to_string(), "oʊ".to_string()];
        let result = converter.convert_ipa_to_espeak(&ipa);

        if let Err(e) = &result {
            eprintln!("Conversion error: {:?}", e);
        }

        assert!(result.is_ok(), "Failed to convert IPA to espeak");

        let espeak_str = result.unwrap();
        eprintln!("Converted: {:?} -> {}", ipa, espeak_str);

        // Basic check: result should not be empty
        assert!(!espeak_str.is_empty(), "Converted string is empty");
    }
}
