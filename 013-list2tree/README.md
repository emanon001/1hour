# list2tree

リスト形式のツリーを`tree`コマンド風の表示に変換する

## Usage

```console
$ cat sample/list1.md
- 1a
  - 2a
  - 2b
    - 3a
  - 2c
    - 3b

$ cat sample/list1.md | cargo run
1a
├── 2a
├── 2b
│   └── 3a
└── 2c
    └── 3b
```
