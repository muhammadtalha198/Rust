// primitive data types
// scalar types


// integers

// integers are whole numbers without a fractional part 
// they can be signed or unsigned
// signed integers can be negative
// unsigned integers can only be positive
// signed integers range from -2^(n-1) to 2^(n-1) - 1
// unsigned integers range from 0 to 2^n - 1

// i8, i16, i32, i64, i128
// u8, u16, u32, u64, u128  

fn integer_types() {
    let i8: i8 = -127;
    let i16: i16 = 32767;
    let i32: i32 = 2147483647;
    let i64: i64 = 9223372036854775807;
    let i128: i128 = 170141183460469231731687303715884105727;

    println!("i8: {}", i8);
    println!("i16: {}", i16);
    println!("i32: {}", i32);
    println!("i64: {}", i64);
    println!("i128: {}", i128);
}

fn unsigned_integer_types() {
    let u8: u8 = 255;
    let u16: u16 = 65535;
    let u32: u32 = 4294967295;
    let u64: u64 = 18446744073709551615;
    let u128: u128 = 340282366920938463463374607431768211455;

    println!("u8: {}", u8);
    println!("u16: {}", u16);
    println!("u32: {}", u32);
    println!("u64: {}", u64);
    println!("u128: {}", u128);
}

fn main() {
    integer_types();
    unsigned_integer_types();
}
