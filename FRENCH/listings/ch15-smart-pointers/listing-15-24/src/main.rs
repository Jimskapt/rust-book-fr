#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let valeur = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&valeur), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *valeur.borrow_mut() += 10;

    println!("a après les opérations = {:?}", a);
    println!("b après les opérations = {:?}", b);
    println!("c après les opérations = {:?}", c);
}
