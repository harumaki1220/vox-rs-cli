import { execa } from "execa";

async function main() {
  console.log("--- TSからRustを呼び出すテスト ---");

  try {
    const rustPath = "../vox-rs-cli/target/debug/vox-rs-cli";

    const { stdout } = await execa(rustPath, ["--list"]);

    console.log("成功!Rustからの回答：\n");
    console.log(stdout);
  } catch (error) {
    console.error("Rustツールの呼び出しに失敗");
    console.error("パスが間違っているか、Rustをビルドしていない可能性");
    console.error(error);
  }
}

main();
