fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(x: i32, label: char) {
    println!("The measurement is: {x}{label}");
}
