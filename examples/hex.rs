use delog::hex_str;

fn main() {
    let buf = [1u8, 2, 3, 0xA1, 0xB7, 0xFF, 0x3];
    println!("'{}'", hex_str!(&buf));
    println!("'{}'", hex_str!(&buf, 2));
    println!("'{:02x}'", hex_str!(&buf, 2));
    println!("'{}'", hex_str!(&buf, 4));
    println!("'{}'", hex_str!(&buf[..], 4));
}
