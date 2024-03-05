use bytes::Buf;

fn main() {
    let a: Vec<u8> = vec![1, 2, 1, 2];
    let mut buffer = &a[..];
    let t = buffer.get_u16();
    println!("{t:?}");
    let len = buffer.get_u16();
    println!("{len:?}");
}
