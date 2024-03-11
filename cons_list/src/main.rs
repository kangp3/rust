#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(5, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(2, Box::new(Nil))))))));
    println!("List is: {:?}", list);
}
