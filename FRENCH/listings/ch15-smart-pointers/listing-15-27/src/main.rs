// ANCHOR: here
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Noeud {
    valeur: i32,
    enfants: RefCell<Vec<Rc<Noeud>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let feuille = Rc::new(Noeud {
        valeur: 3,
        enfants: RefCell::new(vec![]),
    });

    let branche = Rc::new(Noeud {
        valeur: 5,
        enfants: RefCell::new(vec![Rc::clone(&feuille)]),
    });
}
// ANCHOR_END: there
