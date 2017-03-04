#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let object = { MutOrNot("foo".to_string()) };
    subject(object);
}
pub fn subject(mut a: MutOrNot) {
    a.0.push_str("bar");
    println!("subject: {:?}", a);
}
