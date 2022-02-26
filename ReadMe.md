# RustでAtCoder

## 使い方

- コンテストを追加
  - `cargo compete new ${contest_name}` でコンテストを追加
- テスト
  - `cd ${contest_name}`
  - `cargo compete test ${problem_num}`

## 入力の受け取り

### 基本

``` input.rs
input! {
    n: usize,
    mut m: usize,  // 変更可能にする
}
```

### 配列の入力

``` array.rs
input! {
    n: usize,
    mut m: usize,
    v: [usize; n], // 長さnのusizeの配列
    a: [[i32; n]; m] // `a` is Vec<Vec<i32>>, (m, n)-matrix.
}
```

### 文字列の入力

```string.rs
use proconio::input;
use proconio::marker::{Bytes, Chars};

input! {
    string: String, // read as String
    chars: Chars,   // read as Vec<char>
    bytes: Bytes,   // read as Vec<u8>
}

// if you enter "string chars bytes" to the stdin, they are like this.
assert_eq!(string, "string");
assert_eq!(chars, ['c', 'h', 'a', 'r', 's']);
assert_eq!(bytes, b"bytes");
```

### グラフ系

```edge.rs
use proconio::input;
use proconio::marker::Usize1;

input! {
    n: usize,
    edges: [(Usize1, Usize1); n], // Usize1にすると1-index
}

// if you enter "4   1 3   3 4   6 1   5 3", the decremented value is stored.
assert_eq!(edges[0], (0, 2));
assert_eq!(edges[1], (2, 3));
assert_eq!(edges[2], (5, 0));
assert_eq!(edges[3], (4, 2));
```

## 基本の型

### 文字(char)

- chars()とかで文字のiter作れる

#### 連結

- vec![‘a’,’i’].into_iter().collect::\<String>(); で行ける
  - charを連結したものがStringだから。

### 文字列

- &strとString
  - &strは、スライス
  - Stringは文字列型
  - なるべくStringを使ったほうがいろんな処理がしやすい
  - でも、”hoge”とかで定義した値は、&strとなる
  - [rust String &str の変換と、文字列 数値 の変換](https://qiita.com/smicle/items/29a4d5d1d14ad7f77f60)

String -> &str

```string_to_str.rs
let str1: String = String::from("abc");
let str2: &str = &str1;
println!("{}", str2);  // "abc"
```

&str -> String

```str_to_string.rs
let str1: &str = "abc";
let str2: String = str1.to_string();
println!("{}", str2);  // "abc"
```

### Vector・配列

#### 初期化

- とりあえず初期化: `let mut vec = Vec::<T>new();`
- 指定した値で初期化: `let mut vec = vec![1, 2, 3];`
- 空で初期化: `let mut vec = vec![];`
- 値を繰り返し使って初期化: `let mut vec = vec![0;5];`

#### 追加

`vec.push(x)`で追加できる

#### ソート

mutにしておいた上で`vec.sort()`でいける

#### 含まれるか?

`["aaa","iii"].contains(“aaa”);`でいける

#### 最大/最小

`vec.iter().min().unwrap()`で取れる

## データ構造

## 処理

## Rustの実行

- main.rs以外も実行したい場合
  - [Rust：Cargoでmain.rs以外のソースファイルのmain()関数を実行する](https://qiita.com/tatsuya6502/items/7c41dd981ffa56bcab99)
  - cargo.tomlの`bin`に値を追加する

## リンク

- [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/)
- [cargo-compete](https://github.com/qryxip/cargo-compete/blob/master/README-ja.md)