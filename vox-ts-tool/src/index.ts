import { execa } from "execa";

async function main() {
  // 実行時の引数を取得（3番目以降がユーザーの入力）
  const text = process.argv[2];

  if (!text) {
    console.log('使用法: npm run dev -- "喋らせたいテキスト"');
    return;
  }

  try {
    const rustPath = "../vox-rs-cli/target/debug/vox-rs-cli";

    console.log(`「${text}」を音声に変換中...`);

    // Rustに引数を渡して実行
    await execa(rustPath, [
      "--text",
      text,
      "--speaker",
      "3",
      "--output",
      "output.wav",
    ]);

    console.log("音声ファイルの生成に成功しました。");
  } catch (error) {
    console.error("エラーが発生しました:", error);
  }
}

main();
