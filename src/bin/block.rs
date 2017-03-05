#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let object = { MutOrNot("foo".to_string()) };
    { // subject
        let a: MutOrNot = object;
        println!("subject: {:?}", a);
        { // helper
            let mut b: &MutOrNot = &a;
            b.0.push_str("bar");
            println!("helper: {:?}", b);
        }
    }
}
