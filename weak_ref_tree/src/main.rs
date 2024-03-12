use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    v: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let mut leaf = Rc::new(Node{
        v: 3,
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(Weak::new()),
    });
    
    let mut branch = Rc::new(Node{
        v: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf strong count: {}, weak count: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("Branch strong count: {}, weak count: {}", Rc::strong_count(&branch), Rc::weak_count(&branch));

    println!("Leaf: {:?}", &leaf);
    println!("Branch: {:?}", &branch);

    println!("Leaf parent: {:?}", &leaf.parent.borrow().upgrade());
    println!("Branch parent: {:?}", &branch.parent.borrow().upgrade());
}
