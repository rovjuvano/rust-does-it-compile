// Use case of lifetimes
#[derive(Debug)]
struct Object {
    value: String,
}
#[derive(Debug)]
struct Wrapper<'a> {
    object: Option<&'a mut Object>,
}
impl<'a> Wrapper<'a> {
    fn bar(&mut self) {
        match self.object {
            Some(ref mut object) => object.value.push_str("bar"),
            _ => (),
        }
    }
    fn baz(&mut self) {
        match self.object {
            Some(ref mut object) => object.value.push_str("baz"),
            _ => (),
        }
    }
}
fn mutate<'a>(object: &'a mut Object) {
    object.value.push_str("foo");
}
fn wrap<'a>(object: &'a mut Object, wrapper: &mut Wrapper<'a>) {
    wrapper.object = Some(object);
}
fn main() {
    let mut object = Object { value: "".to_string() };
    mutate(&mut object); // mutate<'a> begins and ends here
    {
        let mut wrapper = Wrapper { object: None }; // Wrapper<'a> begins here ...
        wrap(&mut object, &mut wrapper);
        wrapper.bar();
        wrapper.baz();
        println!("{:?}", wrapper);
    } // ... Wrapper<'a> ends here
    println!("{:?}", object);
}
