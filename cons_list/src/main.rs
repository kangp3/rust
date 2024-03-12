#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Cons(2, Rc::new(Nil)))))));
    println!("Ref count on creation is: {}", Rc::strong_count(&a));
    let b = Cons(5, Rc::clone(&a));
    println!("Ref count after constructing list b is: {}", Rc::strong_count(&a));
    {
        let c = Cons(6, Rc::clone(&a));
        println!("Ref count after constructing list c is: {}", Rc::strong_count(&a));
    }
    println!("Ref count after dropping list c is: {}", Rc::strong_count(&a));
    println!("List a is: {:?}", a);
    println!("List b is: {:?}", b);
    // println!("List c is: {:?}", c);
}
