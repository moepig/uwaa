# uwaa
Slack チャンネルに投稿するやつ

## 環境変数
- `UWAA_TOKEN`：Slack の User Token or Bot Token
- `UWAA_CHANNEL`：メッセージを送信する Slack チャンネルの ID

## 実行
```
cargo run --release -- <message>
```