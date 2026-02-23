use anyhow::Result;
use serde::Deserialize;

// 1. APIから返ってくるJSONの形を「構造体」で定義する
// これがRustの「型安全」の基本です。
#[derive(Debug, Deserialize)]
struct Speaker {
    name: String,
    speaker_uuid: String,
}

#[tokio::main] // 2. 非同期処理を動かすための魔法の言葉
async fn main() -> Result<()> {
    // 3. アクセスするURL
    let url = "http://localhost:50021/speakers";

    println!("VOICEVOXからキャラクターリストを取得中...");

    // 4. HTTPリクエストを送る
    // .await を忘れると「未来の約束（Future）」だけで終わってしまいます
    let response = reqwest::get(url).await?;

    // 5. JSONを構造体のリスト（Vec）に変換する
    // ここで定義した Speaker 構造体の形に自動で当てはめてくれます
    let speakers: Vec<Speaker> = response.json().await?;

    println!("--- 登録されているキャラクター一覧 ---");

    // 6. 取得したリストをループで表示
    for speaker in speakers {
        println!("キャラ名: {}, UUID: {}", speaker.name, speaker.speaker_uuid);
    }

    Ok(())
}
