#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let object = { MutOrNot("foo".to_string()) };
    subject(&object);
}
pub fn subject(mut a: &MutOrNot) {
    println!("subject: {:?}", a);
    helper(&mut a);
}
pub fn helper(mut b: &mut MutOrNot) {
    b.0.push_str("bar");
    println!("helper: {:?}", b);
}
