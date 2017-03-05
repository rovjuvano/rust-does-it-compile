fn main() {
    let subject = &mut "foo".to_string();
    subject.push_str("bar");
    subject.push_str("baz");
    subject.push_str("quux");
    println!("subject={:?}", subject);
}
