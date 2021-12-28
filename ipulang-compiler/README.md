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
- [ ] 構文追加
    - [ ] if
    - [ ] for
- [ ] include

### Day2
- [ ] LSP
- [ ] syntax highlight
- [ ] web assembly
- [ ] 型を導入する
    - [ ] 文字列型の導入
    - [ ] ポインタ型の導入
    - [ ] 配列型の導入
- [ ] (error tracing)

## マイルストーン
- [x] 定数, 四則演算が出来る
    - [x] cmmを理解する
    - [x] pl0を理解する
    - [x] 定数をパースできる
    - [x] 四則演算をパースできる
    - [x] 変数を宣言, 使用できる
    - [x] main関数を宣言できる
- [x] LLVM IRコード生成

### LLVM fix
```shell
apt install libclang-common-12-dev llvm-12
```
