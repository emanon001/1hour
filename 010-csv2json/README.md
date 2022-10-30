# csv2json

CSVをJSONに変換する。

## Usage

CSVの内容
```
$ cat samples/liz-bluebird.csv
氏名,担当楽器
鎧塚みぞれ,オーボエ
傘木希美,フルート
```

CSVからJSONに変換
```
$ cat samples/liz-bluebird.csv | deno run mod.ts | jq '.'
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
