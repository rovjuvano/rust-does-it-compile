fn main() {
    let mut subject = &mut "foo".to_string();
    {
        let mut string = &mut subject;
        string.push_str("bar");
        {
            let mut string = &mut string;
            string.push_str("baz");
            {
                let mut string = &mut string;
                string.push_str("quux");
            }
        }
    }
    println!("subject={:?}", subject);
}
