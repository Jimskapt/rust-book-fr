use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn parcourir(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("compteur initial de a = {}", Rc::strong_count(&a));
    println!("prochain élément de a = {:?}", a.parcourir());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("compteur de a après création de b = {}", Rc::strong_count(&a));
    println!("compteur initial de b = {}", Rc::strong_count(&b));
    println!("prochain élément de b = {:?}", b.parcourir());

    if let Some(lien) = a.parcourir() {
        *lien.borrow_mut() = Rc::clone(&b);
    }

    println!("compteur de b après avoir changé a = {}", Rc::strong_count(&b));
    println!("compteur de a après avoir changé a = {}", Rc::strong_count(&a));

    // Décommentez la ligne suivante pour constater que nous sommes dans
    // une boucle de références, cela fera déborder la pile
    // println!("prochain élément de a = {:?}", a.parcourir());
}
// ANCHOR_END: here
