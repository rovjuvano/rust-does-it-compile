fn main() {
    let subject = &mut "foo".to_string();
    bar(&mut subject);
    println!("subject={:?}", subject);
}
fn bar(mut string: &mut String) {
    string.push_str("bar");
    baz(&mut string);
}
fn baz(mut string: &mut String) {
    string.push_str("baz");
    quux(&mut string);
}
fn quux(mut string: &mut String) {
    string.push_str("quux");
}
