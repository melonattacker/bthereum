# 使用方法
## ローカルで実行
### 実行環境
- macOS Catalina 10.15.3
- rustc 1.39.0-beta.9
- cargo 1.39.0-beta

## 実行手順

```
$ git clone https://github.com/melonattacker/bthereum.git
$ cd bthereum/p2p
$ cargo run main.rs server1
```
ターミナルの別タブを開く

```
$ cargo run main.rs server2
```

server1にノード追加要求を送信し、server2がノードリストに追加される

ctrl + Cでserver1にノード削除要求を送信し、server2がノードリストから削除される

## 注意
現時点では2ノード間のノード追加・削除処理しか実装できていません
