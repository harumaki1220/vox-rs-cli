use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about = "VOICEVOX CLI Tool", long_about = None)]
struct Args {
    /// 喋らせたいテキスト
    #[arg(short, long)]
    text: String,

    /// 話者ID (デフォルトはずんだもん: 3)
    #[arg(short, long, default_value_t = 3)]
    speaker: i32,

    /// 出力ファイル名
    #[arg(short, long, default_value = "output.wav")]
    output: String,
}

#[derive(Debug, Deserialize)]
struct Speaker {
    name: String,
    speaker_uuid: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::new();

    // 1. 話者一覧表示
    let speakers = fetch_speakers(&client).await?;
    display_speakers(&speakers);

    println!("\n--- 音声合成を開始します ---");

    // 2. クエリ取得
    let query = create_audio_query(&client, &args.text, args.speaker).await?;
    println!("設計図の取得に成功。");

    // 3. 音声合成
    println!("音声データを生成中...");
    let wav_data = synthesize_voice(&client, &query, args.speaker).await?;

    // 4. ファイル保存
    std::fs::write(&args.output, wav_data)?;
    println!("成功！ '{}' として保存しました。", args.output);

    Ok(())
}

/// VOICEVOX Engineから話者リストを取得する非同期関数
async fn fetch_speakers(client: &reqwest::Client) -> Result<Vec<Speaker>> {
    let url = "http://localhost:50021/speakers";
    println!("VOICEVOXからキャラクターリストを取得中...");

    let response = client.get(url).send().await?;
    let speakers: Vec<Speaker> = response.json().await?;

    Ok(speakers)
}

/// 取得した話者リストをコンソールに表示する関数
/// 引数に & をつけることで「所有権」を奪わずに「貸してもらう（借用）」
fn display_speakers(speakers: &[Speaker]) {
    println!("--- 登録されているキャラクター一覧 ---");
    for speaker in speakers {
        println!("キャラ名: {}, UUID: {}", speaker.name, speaker.speaker_uuid);
    }
}

/// 音声合成の設計図（AudioQuery）を取得する
async fn create_audio_query(
    client: &reqwest::Client,
    text: &str,
    speaker_id: i32,
) -> Result<Value> {
    let url = "http://localhost:50021/audio_query";

    // APIドキュメント通りのクエリパラメータを設定
    let response = client
        .post(url)
        .query(&[("text", text), ("speaker", &speaker_id.to_string())])
        .send()
        .await?;

    // レスポンスを「なんでも入るJSON型（Value）」として受け取る
    let query_json: Value = response.json().await?;

    Ok(query_json)
}

/// 設計図（AudioQuery）を音声データ（WAV）に変換する関数
async fn synthesize_voice(
    client: &reqwest::Client,
    query: &Value,
    speaker_id: i32,
) -> Result<Vec<u8>> {
    let url = "http://localhost:50021/synthesis";

    let response = client
        .post(url)
        .query(&[("speaker", &speaker_id.to_string())])
        .json(query) // 取得した設計図をそのまま投げつける
        .send()
        .await?;

    // レスポンスのバイナリ（バイト列）をそのまま取得
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
