use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();

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
                println!("Add {} to {}!", &name, &department);
                directory.entry(department).and_modify(|employees| { employees.push(name.clone()); employees.sort() }).or_insert(vec!(name.clone()));
            }
            "list" => {
                if words.len() > 2 {
                    println!("List command malformed (ex: 'List', 'List Engineering')");
                }
                match words.get(1) {
                    Some(department) => {
                        match directory.get(*department) {
                            Some(emps) => {
                                for name in emps {
                                    println!("{name}");
                                }
                            }
                            None => println!("Department {department} not found"),
                        }
                    }
                    None => {
                        let mut keys: Vec<&String> = directory.keys().collect();
                        keys.sort();
                        for department in keys {
                            println!("{department}");
                            for name in &directory[department] {
                                println!("  {name}");
                            }
                        }
                    }
                }
            }
            cmd => println!("Command '{cmd}' not recognized"),
        }
    }
}
