use std::ops::Deref;
use std::fmt::Display;

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(v: T) -> Self {
        MyBox(v)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data {}", &self.0);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    // drop(y);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}
