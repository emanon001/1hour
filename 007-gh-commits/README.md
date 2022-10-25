# gh-commits

[Github GraphQL API](https://docs.github.com/ja/graphql)を使用して、指定した日付のコミットを出力する。  
出力形式はScrapboxの箇条書き形式。

## Usage

今日のコミットを出力。
```
$ GH_COMMITS_AUTH_TOKEN=XXX GH_COMMITS_EMAIL=XXX deno run --allow-net --allow-env mod.ts
 [1hour https://github.com/emanon001/1hour] 3commits
  [chore: .gitignoreを追加 https://github.com/emanon001/1hour/commit/bee1eecfc40da3ac95f52e4a1100574cba951817]
  [fix: remove debug code https://github.com/emanon001/1hour/commit/157872f4d91b9b706cc73c1c7b87a548a2cfdfc8]
  [feat: 複数ブランチ対応 https://github.com/emanon001/1hour/commit/3a5e51da7cd55469fddabb3c2dd37d7fcbab9345]
```

指定した日付のコミットを出力。
```
$ GH_COMMITS_AUTH_TOKEN=XXX GH_COMMITS_EMAIL=XXX deno run --allow-net --allow-env mod.ts 2022-10-26
 [1hour https://github.com/emanon001/1hour] 3commits
  [chore: .gitignoreを追加 https://github.com/emanon001/1hour/commit/bee1eecfc40da3ac95f52e4a1100574cba951817]
  [fix: remove debug code https://github.com/emanon001/1hour/commit/157872f4d91b9b706cc73c1c7b87a548a2cfdfc8]
  [feat: 複数ブランチ対応 https://github.com/emanon001/1hour/commit/3a5e51da7cd55469fddabb3c2dd37d7fcbab9345]
```
