# ipulang
- まずはレジスタ型でなくスタック型  
- まずは型はintだけ

## やることリスト
### Day1
- [x] nomの理解
- [x] CLIにする
    - [x] `cargo run -- <input> --output <output>`
- [x] nomでASTにする
- [x] LLVM IRを理解する
    - `inkwell` を使う
- [x] 変数機能
    - [x] parser
    - [x] node
    - [x] codegen
    - [x] 変数初期化
    - [x] assign
- [x] 関数宣言
    - [x] parser
    - [x] node
    - [x] codegen
    - [x] main関数宣言
    - [x] main以外の関数宣言
    - [x] return文
    - [x] 関数呼び出し
        - [x] 引数付きで呼び出す
          - [x] 引数のallocaをまだやっていない
- [x] declare
    - [x] putchar

### Day2
- [x] 構文追加
    - [x] if
    - [x] for
- [x] 型を導入する
    - [x] i32, i64, bool
        - [ ] astに型の情報を入れる
### Day3?
    - [ ] 文字列型の導入
    - [ ] ポインタ型の導入
    - [ ] 配列型の導入
- [ ] include
- [ ] (error tracing)
- [ ] parser書き直す?
    - [ ] row, columnの情報が欲しい
- [ ] REPL
- [ ] 入力
- [ ] コメント
- [ ] 構造体


### やらないこと
- [ ] LSP
    - hard
    - [ ] syntax highlight
- [ ] formatter -> ASTを文字列化してそう
- [ ] web assembly

## マイルストーン
- [x] 定数, 四則演算が出来る
    - [x] cmmを理解する
    - [x] pl0を理解する
    - [x] 定数をパースできる
    - [x] 四則演算をパースできる
    - [x] 変数を宣言, 使用できる
    - [x] main関数を宣言できる
- [x] LLVM IRコード生成
- [x] fizzbuzz

### LLVM fix
```shell
apt install libclang-common-12-dev llvm-12
```
