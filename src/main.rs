use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about = "VOICEVOX CLI Tool", long_about = None)]
struct Args {
    /// 喋らせたいテキスト
    #[arg(short, long)]
    text: Option<String>, // 一覧表示だけの時もあるので Option にする

    /// 話者ID (デフォルトはずんだもん: 3)
    #[arg(short, long, default_value_t = 3)]
    speaker: i32,

    /// 出力ファイル名
    #[arg(short, long, default_value = "output.wav")]
    output: String,

    /// 話者一覧を表示する
    #[arg(short, long)]
    list: bool,
}

#[derive(Debug, Deserialize)]
struct Style {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Speaker {
    name: String,
    styles: Vec<Style>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::new();

    // --list が指定された場合
    if args.list {
        let speakers = fetch_speakers(&client).await?;
        display_speakers(&speakers);
        return Ok(()); // 一覧を出したら終了する
    }

    // テキストが指定されていない場合はエラーを出す
    let text = match args.text {
        Some(t) => t,
        None => {
            anyhow::bail!("テキストを指定してください。例: --text \"こんにちは\"");
        }
    };

    println!("--- 音声合成を開始します ---");

    // クエリ取得
    let query = create_audio_query(&client, &text, args.speaker).await?;

    // 音声合成
    let wav_data = synthesize_voice(&client, &query, args.speaker).await?;

    // ファイル保存
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
fn display_speakers(speakers: &[Speaker]) {
    println!("--- 利用可能なキャラクターとID一覧 ---");
    for speaker in speakers {
        for style in &speaker.styles {
            println!("[{:>3}] {}: {}", style.id, speaker.name, style.name);
        }
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
