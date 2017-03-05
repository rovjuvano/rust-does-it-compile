#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let mut object = { MutOrNot("foo".to_string()) };
    subject(&mut object);
}
pub fn subject(a: &mut MutOrNot) {
    println!("subject: {:?}", a);
    helper(&mut a);
}
pub fn helper(b: &mut MutOrNot) {
    b.0.push_str("bar");
    println!("helper: {:?}", b);
}
