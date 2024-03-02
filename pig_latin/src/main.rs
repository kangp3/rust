const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

fn main() {
    let start_str = "Hello and end each world";
    println!("Translating '{start_str}' to pig latin:\n -> {}", pig_latin(start_str));
}

fn pig_latin(s: &str) -> String {
    let mut pig_latinized = String::new();
    for word in s.split_whitespace() {
        if word.starts_with(VOWELS) {
            pig_latinized.push_str(&format!(" {word}-hay"));
        } else {
            let mut chars = word.chars();
            let first = chars.next().expect("word should have first character");
            pig_latinized.push_str(&format!(" {}-{first}ay", chars.collect::<String>()));
        }
    }
    pig_latinized
}
