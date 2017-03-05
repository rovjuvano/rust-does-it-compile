fn main() {
    let subject = &mut "foo".to_string();
    bar(subject);
    println!("subject={:?}", subject);
}
fn bar(string: &mut String) {
    string.push_str("bar");
    baz(string);
}
fn baz(string: &mut String) {
    string.push_str("baz");
    quux(string);
}
fn quux(string: &mut String) {
    string.push_str("quux");
}
