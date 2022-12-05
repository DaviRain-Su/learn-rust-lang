fn main() {
    let value_u8 = 255u8;
    println!("display {}", value_u8); // 255
    // debug will report this error overflow
    // release will (255 + 1) % mod 256
    println!("display (255 + 1) is {}", value_u8 + 1); // 0
}