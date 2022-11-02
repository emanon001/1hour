# json2csv

JSONをCSVに変換する。
[csv2json](../010-csv2json/)の逆パターン。

## Usage

JSONの内容
```
$ cat samples/liz-bluebird.json
[
  {
    "氏名": "鎧塚みぞれ",
    "担当楽器": "オーボエ"
  },
  {
    "氏名": "傘木希美",
    "担当楽器": "フルート"
  }
]
```

JSONからCSVに変換
```
$ cat samples/liz-bluebird.json | deno run
氏名,担当楽器
鎧塚みぞれ,オーボエ
傘木希美,フルート
```
