fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");

    let mut count = 0;
    let result = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up "asdf";
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count}");
    println!("result = {result}");

    let mut count = 3;
    while count != 0 {
        println!("{count}!");
        count -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for i in a {
        println!("the value is: {i}");
    }

    for i in (1..4).rev() {
        println!("{i}!");
    }
    println!("LIFTOFF!!!");
}
