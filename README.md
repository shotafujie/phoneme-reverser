# phoneme-reverser

音素選択TUIで逆再生音声を生成するツール。

キーボードで音素を選択し、正順・逆順で音声合成して再生・保存できます。通常の波形逆再生と異なり、各音素は自然に発音されるため「発音可能な逆さ言葉」が生成されます。

## 概要

```
音素選択: [a] [k] [a]
  ↓
正順: /a/ /k/ /a/ → "あかあ"
逆順: /a/ /k/ /a/ → "あかあ"（回文なので同じ）
```

バックマスキング風だが明瞭に聞き取れる、ネタ寄りの音声エフェクトツール。

## 技術スタック

- **Rust** - メイン実装
- **ratatui** - ターミナルUI
- **crossterm** - ターミナル制御・イベント処理
- **PyO3** - Pythonバインディング（lexconvert呼び出し）
- **espeak-ng** - 音素列から音声合成
- **CPAL** - 音声再生
- **chrono** - タイムスタンプ生成

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

### TUI起動

```bash
cargo run
```

### キーボードショートカット

#### 音素選択画面

| キー | 説明 |
|------|------|
| `a`, `i`, `u`, `e`, `o` | 日本語母音を選択 |
| `@`, `A`, `O` | 英語母音を選択 |
| `p`, `b`, `t`, `d`, `k`, `g`, `m`, `n`, etc. | 子音を選択 |
| `Backspace` | 最後の音素を削除 |
| `Enter` | プレビュー画面へ移動 |
| `q` | 終了 |

#### プレビュー画面

| キー | 説明 |
|------|------|
| `p` | 正順音声を再生 |
| `r` | 逆順音声を再生 |
| `s` | 逆順音声をタイムスタンプ形式で保存（例: `20260111123456.wav`） |
| `Esc` | 音素選択画面に戻る |
| `q` | 終了 |

### サポート音素

#### 母音（8個）

- **日本語**: `a` (あ), `i` (い), `u` (う), `e` (え), `o` (お)
- **英語**: `@` (about), `A` (father), `O` (thought)

#### 子音（20個）

- **破裂音**: `p` (ぱ), `b` (ば), `t` (た), `d` (だ), `k` (か), `g` (が)
- **鼻音**: `m` (ま), `n` (な), `N` (sing)
- **摩擦音**: `s` (さ), `z` (ざ), `S` (しゃ), `Z` (vision), `h` (は), `f` (fan), `v` (van)
- **側音/ふるえ音**: `l` (light), `r` (巻き舌)
- **接近音**: `w` (わ), `y` (や)

