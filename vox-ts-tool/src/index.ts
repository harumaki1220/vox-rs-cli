import process from "node:process";
import { execa } from "execa";

async function main() {
  // 全ての引数を取得
  const args = process.argv.slice(2);
  const rustPath = "../vox-rs-cli/target/debug/vox-rs-cli";

  // --list が含まれているかチェック
  if (args.includes("--list") || args.includes("-l")) {
    try {
      const { stdout } = await execa(rustPath, ["--list"]);
      console.log(stdout);
      return;
    } catch (error) {
      console.error("リストの取得に失敗しました:", error);
      return;
    }
  }

  // --list がない場合は、最初の引数を「喋らせるテキスト」として扱う
  const text = args[0];

  if (!text) {
    console.log("使用法:");
    console.log("  話者一覧: npm run dev -- --list");
    console.log('  音声生成: npm run dev -- "こんにちは"');
    return;
  }

  try {
    console.log(`「${text}」を音声に変換中...`);

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
