fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {s1}, s2: {s2}");

    let s = String::from("hello");
    let x = 5;
    takes_ownership(s);
    makes_copy(x);
    println!("s: {s}, x: {x}");
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn makes_copy(x: i64) {
    println!("{x}");
}
