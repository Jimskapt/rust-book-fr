// ANCHOR: here
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Noeud {
    valeur: i32,
    parent: RefCell<Weak<Noeud>>,
    enfants: RefCell<Vec<Rc<Noeud>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let feuille = Rc::new(Noeud {
        valeur: 3,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![]),
    });

    println!("parent de la feuille = {:?}", feuille.parent.borrow().upgrade());

    let branche = Rc::new(Noeud {
        valeur: 5,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![Rc::clone(&feuille)]),
    });

    *feuille.parent.borrow_mut() = Rc::downgrade(&branche);

    println!("parent de la feuille = {:?}", feuille.parent.borrow().upgrade());
}
// ANCHOR_END: there
