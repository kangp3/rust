fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let f = five();
    println!("The value of f is: {f}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(x: i32, label: char) {
    println!("The measurement is: {x}{label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
