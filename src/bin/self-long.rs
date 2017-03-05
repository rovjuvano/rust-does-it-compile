#[derive(Debug)]
pub struct MutOrNot(String);
fn main() {
    let mut object = { MutOrNot("foo".to_string()) };
    object.subject();
}
impl MutOrNot {
    pub fn subject(mut self: &mut Self) {
        println!("subject: {:?}", self);
        self.helper();
    }
    pub fn helper(mut self: &mut Self) {
        self.0.push_str("bar");
        println!("helper: {:?}", self);
    }
}
