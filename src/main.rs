fn main() {
    let mut subject = "foo".to_string();
    bar(&mut subject);
    println!("subject={:?}", subject);
}
fn bar(string: &mut String) {
    string.push_str("bar");
    baz(&mut string);
}
fn baz(string: &mut String) {
    string.push_str("baz");
    quux(&mut string);
}
fn quux(string: &mut String) {
    string.push_str("quux");
}
