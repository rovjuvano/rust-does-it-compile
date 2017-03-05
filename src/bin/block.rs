#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let object = { MutOrNot("foo".to_string()) };
    { // subject
        let mut a: &mut MutOrNot = &mut object;
        println!("subject: {:?}", a);
        { // helper
            let mut b: &mut MutOrNot = &mut a;
            b.0.push_str("bar");
            println!("helper: {:?}", b);
        }
    }
}
