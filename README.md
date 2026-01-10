# phoneme-reverser

音素分割逆再生音声合成ツール。

入力音声を音素単位で分割し、逆順に並び替えてから再合成する。通常の波形逆再生と異なり、各音素は自然に発音されるため「発音可能な逆さ言葉」が生成される。

## 概要

```
"さくら" → /s/ /a/ /k/ /u/ /r/ /a/ → /a/ /r/ /u/ /k/ /a/ /s/ → "あるかす"
```

バックマスキング風だが明瞭に聞き取れる、ネタ寄りの音声エフェクトツール。

## 技術スタック

- **Rust** - メイン実装
- **PyO3** - PythonバインディングでAllosaurus呼び出し
- **Allosaurus** - 音声から直接音素を認識（2000言語以上対応）
- **espeak-ng** - 音素列から音声合成
- **CPAL** - 音声再生

## ファイル構成

```
phoneme-reverser/
├── Cargo.toml
├── src/
│   ├── main.rs         # エントリーポイント、CLI
│   ├── phoneme.rs      # Allosaurus呼び出し (PyO3)
│   ├── synth.rs        # espeak-ng合成
│   └── audio.rs        # WAV読み書き、再生 (CPAL)
├── pyproject.toml      # Python依存 (allosaurus)
└── README.md
```

## 依存クレート

```toml
[dependencies]
pyo3 = { version = "0.22", features = ["auto-initialize"] }
cpal = "0.15"
hound = "3.5"
anyhow = "1.0"
clap = { version = "4", features = ["derive"] }
```

## 基本フロー

```
input.wav
    ↓
[Allosaurus] 音声→音素列
    ↓
/æ/ /l/ /u/ /s/ /ɔ/ /ɹ/ /s/
    ↓
逆順
    ↓
/s/ /ɹ/ /ɔ/ /s/ /u/ /l/ /æ/
    ↓
[espeak-ng] 音素→音声合成
    ↓
output.wav + 再生
```

## セットアップ

### Python環境

```bash
pip install allosaurus
# 初回実行でモデルがダウンロードされる
python -m allosaurus.run -i sample.wav
```

### ビルド

```bash
cargo build --release
```

## 使い方

```bash
phoneme-reverser input.wav -o output.wav
```

