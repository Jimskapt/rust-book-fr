enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("compteur après la création de a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("compteur après la création de b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("compteur après la création de c = {}", Rc::strong_count(&a));
    }
    println!("compteur après que c soit sortie de la portée = {}", Rc::strong_count(&a));
}
// ANCHOR_END: here
