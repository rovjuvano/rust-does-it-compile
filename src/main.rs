#[derive(Debug)]
struct MyType {
    a: String,
    b: u8,
    c: Vec<u8>,
}
fn main() {
    let x = MyType { a: "String".to_string(), b: 8u8, c: vec![1, 2, 3] };
    fun(x);
}
fn fun(MyType { a, b, c }: MyType) {
    println!("a: {:?}\nb: {:?}\nc: {:?}", a, b, c);
}
