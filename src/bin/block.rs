#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let mut object = { MutOrNot("foo".to_string()) };
    { // subject
        let mut a: MutOrNot = object;
        a.0.push_str("bar");
        println!("subject: {:?}", a);
    }
}
