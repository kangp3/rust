fn main() {
    let s = String::from("Hello world");
    let f = first_word(&s);
    println!("First word is: {f}");
}

fn first_word(s: &str) -> &str {
    let b = s.as_bytes();
    for (i, &c) in b.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    s
}
