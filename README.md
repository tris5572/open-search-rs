
コマンドライン引数として渡された（exeファイルにドロップされた）ファイルについて、そのファイル名のGoogle検索結果をWebブラウザで開く簡易アプリです。

# 動作環境

Windows

# 開発時の試験実行

試験のために実行するときは、以下のようなコマンドによりコマンドライン引数としてファイル名を渡すとスムーズにテスト実行できます。

```shell
cargo run -- "D:\\Download Files\\試験 ファイル.zip"
```

# メモ

コマンドプロンプトではファイル名に空白を含む場合に `start` コマンドへ上手く引数を渡せなかったため、PowerShellを使用しています。
