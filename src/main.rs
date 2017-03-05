fn main() {
    let subject = &mut "foo".to_string();
    (&mut* subject).push_str("bar");
    (&mut* subject).push_str("baz");
    (&mut* subject).push_str("quux");
    println!("subject={:?}", subject);
}
