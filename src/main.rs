use std::path::PathBuf;

/// 使用する検索エンジンのURL。
/// この文字列に引き続いて、キーワード(ファイル名)が指定される。
const SEARCH_URL: &str = "https://duckduckgo.com/?t=h_&ia=web&q=";
// const SEARCH_URL: &str = "http://www.google.com/search?q=";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // ファイルが指定されなかったときは何もせず終了する
    if args.len() <= 1 {
        return;
    }

    let path = PathBuf::from(&args[1]);

    // 拡張子を除いたファイル名を取得
    let name = match path.file_stem() {
        Some(v) => v,
        None => return,
    };
    let name = name.to_str().unwrap().to_string();

    // キーワードを含めたURLを生成
    let url = format!(r#""{}{}""#, SEARCH_URL, name);

    // 外部コマンドを実行
    let _ = std::process::Command::new("powershell")
        .args(["Start-Process", &url])
        .output()
        .expect("failed to execute process");

    // デバッグ用
    // println!("{}", &url);
}
