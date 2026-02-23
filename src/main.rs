use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Speaker {
    name: String,
    speaker_uuid: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. データを取ってくる（非同期関数）
    let speakers = fetch_speakers().await?;

    // 2. データを表示する（普通の関数）
    display_speakers(&speakers);

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
