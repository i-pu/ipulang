# ipulang
- まずはレジスタ型でなくスタック型  
- まずは型はintだけ

## やることリスト
- [x] nomの理解
- [x] CLIにする
    - [x] `cargo run -- <input> --output <output>`
- [x] nomでASTにする
- [x] LLVM IRを理解する
    - `inkwell` を使う
- [] 変数機能
    - [x] parser
    - [x] node
    - [ ] codegen
- [] 関数宣言
    - [x] parser
    - [x] node
    - [ ] codegen

## マイルストーン
- [x] 定数, 四則演算が出来る
    - [x] cmmを理解する
    - [x] pl0を理解する
    - [x] 定数をパースできる
    - [x] 四則演算をパースできる
    - [x] 変数を宣言, 使用できる
    - [x] 関数を宣言できる
    
- [ ] LLVM IRコード生成

### LLVM IR勉強会
```shell
apt install libclang-common-12-dev llvm-12
```

```

```