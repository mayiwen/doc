use std::marker::Tuple;

fn main() {}

#[test]
fn test01() {
    // u32 就是数据类型
    let guess: u32 = "42".parse().expect("Not a number");
}
#[test]
fn test02() {
    // 标量类型

    // 数字
    let i8: i8 = -1;
    let u8: u8 = 1;
    let i16: i16 = -1;
    let u16: u16 = 1;
    let i32: i32 = -1;
    let u32: u32 = 1;
    let i64: i64 = -1;
    let u64: u64 = 1;
    let i128: i128 = -1;
    let u128: u128 = 1;
    let isize: i128 = -1;
    let usize: u128 = 1;

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let dinary = 0b1111_0000;
    let bye = b'A';
}

fn float() {
    let x: f32 = 2.0;
    let y: f64 = 3.0;
    let z = x as f64 + y;
    let z = x as f64 - y;
    let z = x as f64 * y;
    let z = x as f64 / y;
    let z = x as f64 % y;
}

fn bool(真或假: bool) -> bool {
    let 真: bool = true;
    let 假 = false;
    if 真或假 { 真 } else { 假 }
}

fn char() {
    let a = '你';
    let a = '⊕';
}

// 复合类型

fn tuple() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    tuple.0;
    tuple.1;
    tuple.2;
    // tuple.3;

    let (x, y, z) = tuple; // 解构
}
