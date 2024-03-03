use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut directory: HashMap<&str, Vec<&str>> = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("stdout should flush");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("stdin should be read");

        let words: Vec<&str> = buf.split_whitespace().collect();
        match words.get(0).expect("input should have command").to_lowercase().as_ref() {
            "add" => {
                if words.len() != 4 || words[2] != "to" {
                    println!("Add command malformed (ex: 'Add Sally to Engineering')");
                    continue;
                }
                let name = String::from(words[1]);
                let department = String::from(words[3]);
                directory.entry(&department).and_modify(|employees| { employees.push(&name) }).or_insert(vec!());
                println!("Add {} to {}!", &name, &department);
            }
            "list" => println!("List!"),
            cmd => println!("Command '{cmd}' not recognized"),
        }
    }
}
