#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let object = { MutOrNot("foo".to_string()) };
    object.subject();
}
impl MutOrNot {
    pub fn subject(mut self) {
        self.0.push_str("bar");
        println!("subject: {:?}", self);
    }
}
