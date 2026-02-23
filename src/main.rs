use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Speaker {
    name: String,
    speaker_uuid: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 話者一覧表示
    let speakers = fetch_speakers().await?;
    display_speakers(&speakers);

    println!("\n--- 音声合成を開始します ---");

    let text = "初めまして、ずんだもんなのだ！";
    let speaker_id = 3; // ずんだもん

    // 2. クエリ取得
    let query = create_audio_query(text, speaker_id).await?;
    println!("設計図の取得に成功。");

    // 3. 音声合成
    println!("音声データを生成中...");
    let wav_data = synthesize_voice(&query, speaker_id).await?;

    // 4. ファイル保存
    let file_name = "zundamon.wav";
    std::fs::write(file_name, wav_data)?;
    println!("成功！ '{}' として保存しました。", file_name);

    Ok(())
}

/// VOICEVOX Engineから話者リストを取得する非同期関数
async fn fetch_speakers() -> Result<Vec<Speaker>> {
    let url = "http://localhost:50021/speakers";
    println!("VOICEVOXからキャラクターリストを取得中...");

    let response = reqwest::get(url).await?;
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
async fn create_audio_query(text: &str, speaker_id: i32) -> Result<Value> {
    let client = reqwest::Client::new(); // POSTにはクライアントが必要

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
async fn synthesize_voice(query: &Value, speaker_id: i32) -> Result<Vec<u8>> {
    let client = reqwest::Client::new();
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
