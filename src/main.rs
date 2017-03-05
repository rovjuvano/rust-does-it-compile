fn main() {
    let subject = &mut "foo".to_string();
    {
        let string = subject;
        string.push_str("bar");
        {
            let string = string;
            string.push_str("baz");
            {
                let string = string;
                string.push_str("quux");
            }
        }
    }
    println!("subject={:?}", subject);
}
