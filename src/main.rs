#[derive(Debug)]
struct MyType {
    a: String,
    b: u8,
    c: Vec<u8>,
}
fn main() {
    let x = &mut MyType { a: "String".to_string(), b: 8u8, c: vec![1, 2, 3] };
    match x {
        &mut MyType { ref mut a, ref b, ref c } => {
            a.push_str(" appended");
            println!("a: {:?}\nb: {:?}\nc: {:?}", a, b, c)
        },
    };
}
