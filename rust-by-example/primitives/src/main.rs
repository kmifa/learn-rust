use std::fmt;
use std::mem;

// sliceを借用する
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 固定長の配列（型シグネチャは冗長なので、なくても可）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // すべての要素を0にする場合
    let ys: [i32; 500] = [0; 500];

    // インデックスは０から
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    // `len`は配列の要素数を返す。
    println!("number of elements in array: {}", xs.len());
    // 配列はスタック上に置かれる
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    // 配列は自動的にスライスとして借用される。
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // スライスは配列の一部を指すことができる。
    // [starting_index..ending_index] の形をとり、
    // starting_index はスライスの先頭の位置を表し、
    // ending_index はスライスの末尾の1つ先の位置を表す。
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // インデックスが範囲外のときはコンパイルエラー
    // println!("{}", xs[5]);
}

// fn reverse(pair: (i32, bool)) -> (bool, i32) {
//     let (integer, boolean) = pair;

//     (boolean, integer)
// }

// fn transpose(matrix: Matrix) -> Matrix {
//     Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
// }

// // `{}` というマーカーを使用するためには、
// // この型専用の`fmt::Display`というトレイトが実装されていなくてはなりません。
// #[derive(Debug)]
// struct Matrix(f32, f32, f32, f32);
// impl fmt::Display for Matrix {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
//     }
// }

// fn main() {
//     // 様々な型を値に持つタプル
//     let long_tuple = (
//         1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
//     );

//     // インデックスを用いて、タプル内の要素を参照できる。
//     println!("long tuple first value: {}", long_tuple.0);
//     println!("long tuple second value: {}", long_tuple.1);

//     // タプルの中にタプルを定義することも出来る
//     let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

//     // タプルはプリント可能である。
//     println!("tuple of tuples: {:?}", tuple_of_tuples);

//     // 長すぎるタプルはプリントできない(最大12個までっぽい)
//     // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     // println!("too long tuple: {:?}", too_long_tuple);

//     let pair = (1, true);

//     println!("pair is {:?}", pair);

//     println!("the reversed pair is {:?}", reverse(pair));

//     // 要素を1つしか持たないタプルを作成する場合、括弧で囲まれたただのリテラル
//     // と区別するため、カンマが必要になる。
//     println!("one element tuple: {:?}", (5u32,));
//     println!("just an integer: {:?}", (5u32));

//     //タプルを分解して別の変数にそれぞれの値を代入
//     let tuple = (1, "hello", 4.5, true);

//     let (a, b, c, d) = tuple;
//     println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

//     let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
//     println!("{}", matrix);

//     println!("Matrix:\n{}", matrix);
//     println!("Transpose:\n{}", transpose(matrix));
// }

// fn main_1() {
//     // # 基本データ型 //
//     let logical: bool = true;

//     let a_float: f64 = 1.0;
//     // サフィックスによる型指定
//     let an_integer = 5i32;

//     // デフォルト
//     let default_float = 3.0;
//     let default_integer = 7;

//     // 型を文脈から推定
//     let mut inferred_type = 12;

//     // 型i64は次行の内容に基づいて推定
//     inferred_type = 4294967296i64;

//     // ミュータブルな変数は値を変更できる
//     let mut mutable = 12;
//     mutable = 21;

//     // 型は不変
//     // mutable = true;

//     // 変数はシャドーイングによって上書きできる
//     let mutable = true;
// }

// fn main_2() {
//     // # リテラルとオペレータ //
//     // 足し算
//     println!("1 + 2 = {}", 1u32 + 2);

//     // 引き算
//     println!("1 - 2 = {}", 1i32 - 2);

//     // 論理演算子
//     println!("true AND false is {}", true && false);
//     println!("true OR false is {}", true || false);
//     println!("NOT true is {}", !true);

//     // ビットワイズ演算
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

//     // 可読性のための`_`（アンダースコア）の使用
//     println!("One million is written as {}", 1_000_000u32);
// }
