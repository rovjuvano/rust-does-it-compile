fn main() {
    let subject = &mut "foo".to_string();
    bar(subject);
    println!("subject={:?}", subject);
}
fn bar(mut string: &mut String) {
    string.push_str("bar");
    baz(string);
}
fn baz(mut string: &mut String) {
    string.push_str("baz");
    quux(string);
}
fn quux(mut string: &mut String) {
    string.push_str("quux");
}
