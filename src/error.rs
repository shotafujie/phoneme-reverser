use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhonemeReverserError {
    #[error("Audio file error: {0}")]
    AudioFile(#[from] hound::Error),

    #[error("Audio playback error: {0}")]
    AudioPlayback(String),

    #[error("Synthesis error: {0}")]
    Synthesis(String),

    #[error("Phoneme recognition error: {0}")]
    PhonemeRecognition(String),

    #[error("Phoneme conversion error: {0}")]
    PhonemeConversion(String),

    #[error("Python initialization error: {0}")]
    PythonInit(#[from] pyo3::PyErr),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PhonemeReverserError>;
