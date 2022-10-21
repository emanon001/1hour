# jq-finder

jqコマンドのフィルタリング結果を確認するTUIツール。

## Usage

```
$ cargo run -- --json-file sample/foo.json
```

![jq-finder1](./jq-finder1.png)

入力したフィルタを適用した結果が出力欄に反映される。
![jq-finder2](./jq-finder2.png)


なお、出力欄のスクロールを実装していないため、JSONが大きいと全体の構造を確認できない。
