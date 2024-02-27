fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let f = five();
    println!("The value of f is: {f}");
}

fn print_labeled_measurement(x: i32, label: char) {
    println!("The measurement is: {x}{label}");
}

fn five() -> i32 {
    5
}
