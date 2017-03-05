fn main() {
    let mut subject = &mut "foo".to_string();
    {
        let string = &mut subject;
        string.push_str("bar");
        {
            let string = &mut string;
            string.push_str("baz");
            {
                let string = &mut string;
                string.push_str("quux");
            }
        }
    }
    println!("subject={:?}", subject);
}
