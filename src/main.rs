// Use case of lifetimes
#[derive(Debug)]
struct Object {
    value: String,
}
#[derive(Debug)]
struct Wrapper<'a> {
    object: &'a mut Object,
}
impl<'a> Wrapper<'a> {
    fn bar(&mut self) {
        self.object.value.push_str("bar");
    }
    fn baz(&mut self) {
        self.object.value.push_str("baz");
    }
}
fn mutate<'a>(object: &'a mut Object) {
    object.value.push_str("foo");
}
fn wrap<'a>(object: &'a mut Object) -> Wrapper<'a> {
    Wrapper { object: object }
}
fn main() {
    let mut object = Object { value: "".to_string() };
    mutate(&mut object); // mutate<'a> begins and ends here
    {
        let mut wrapper = wrap(&mut object); // Wrapper<'a> begins here ...
        wrapper.bar();
        wrapper.baz();
        println!("{:?}", wrapper);
    } // ... Wrapper<'a> ends here
    println!("{:?}", object);
}
