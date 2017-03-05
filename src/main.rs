fn main() {
    let mut subject = "foo".to_string();
    (&mut subject).push_str("bar");
    (&mut (&mut subject)).push_str("baz");
    (&mut (&mut (&mut subject))).push_str("quux");
    println!("subject={:?}", subject);
}
