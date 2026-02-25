# vox-voice-app

TypeScript (Node.js) から Rust 製 CLI ツールを呼び出し、VOICEVOX を利用して音声合成を行うプロジェクトです。

## プロジェクト構成

- `vox-rs-cli/`: Rust 製のコアロジック。VOICEVOX Engine と通信し、WAV ファイルを生成します。
- `vox-ts-tool/`: TypeScript 製のラッパー。コマンドライン引数を処理し、Rust バイナリを制御します。

## セットアップ

### 1. 前提条件

- **VOICEVOX Engine** が起動していること（デフォルト: `localhost:50021`）。
- Rust (Cargo) および Node.js (npm) がインストールされていること。

### 2. Rust 側のビルド

TS から呼び出すための実行ファイルを生成します。

```bash
cd vox-rs-cli
cargo build
```

### 3. TypeScript 側の準備

```bash
cd vox-ts-tool
npm install
```

## 使い方 (TypeScript)

vox-ts-tool ディレクトリ内で実行します。

### 話者一覧の表示

```bash
npm run dev -- --list
```

### 音声の生成

```bash
npm run dev -- "喋らせたいテキスト"
```

※ 成功すると vox-ts-tool/output.wav が生成されます。

## 直接 Rust CLI を使用する場合

```bash
cd vox-rs-cli
# リスト表示
cargo run -- --list
# 音声生成
cargo run -- --text "こんにちは" --speaker 3 --output rust_test.wav
```

## クレジット

音声合成には [VOICEVOX](https://voicevox.hiroshiba.jp/) を使用しています。
生成した音声を利用する際は、各キャラクターの利用規約を遵守してください。
