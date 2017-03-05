#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let object = { MutOrNot("foo".to_string()) };
    subject(object);
}
pub fn subject(a: MutOrNot) {
    println!("subject: {:?}", a);
    helper(a);
}
pub fn helper(mut b: MutOrNot) {
    b.0.push_str("bar");
    println!("helper: {:?}", b);
}
