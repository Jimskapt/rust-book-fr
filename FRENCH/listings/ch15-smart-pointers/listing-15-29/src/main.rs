use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Noeud {
    valeur: i32,
    parent: RefCell<Weak<Noeud>>,
    enfants: RefCell<Vec<Rc<Noeud>>>,
}

// ANCHOR: here
fn main() {
    let feuille = Rc::new(Noeud {
        valeur: 3,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![]),
    });

    println!(
        "feuille strong = {}, weak = {}",
        Rc::strong_count(&feuille),
        Rc::weak_count(&feuille),
    );

    {
        let branche = Rc::new(Noeud {
            valeur: 5,
            parent: RefCell::new(Weak::new()),
            enfants: RefCell::new(vec![Rc::clone(&feuille)]),
        });

        *feuille.parent.borrow_mut() = Rc::downgrade(&branche);

        println!(
            "branche strong = {}, weak = {}",
            Rc::strong_count(&branche),
            Rc::weak_count(&branche),
        );

        println!(
            "feuille strong = {}, weak = {}",
            Rc::strong_count(&feuille),
            Rc::weak_count(&feuille),
        );
    }

    println!("parent de la feuille = {:?}", feuille.parent.borrow().upgrade());
    println!(
        "feuille strong = {}, weak = {}",
        Rc::strong_count(&feuille),
        Rc::weak_count(&feuille),
    );
}
// ANCHOR_END: here
