pub fn run() {
    let i8: i8 = 127; // -2^8 ~ 2^8 -1
    let i32: i32 = 2147483647; // -2^32 ~ 2^32 - 1
    let u8: u8 = 255; // 0 ~ 2^8 - 1
    let u32: u32 = 4294967295; // 0 ~ 2^32 - 1
    println!("{} {} {} {}", i8, i32, u8, u32);

    let f32: f32 = 3.0; // f32
    let f64 = 2.0; // f64
    let t = true;
    let f: bool = false;
    println!("{} {} {} {}", f32, f64, t, f);
}
