fn main() {
    // ANCHOR: here
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("salut"));

    fn prend_un_long_type(f: Thunk) {
        // -- partie masquée ici --
    }

    fn retourne_un_long_type() -> Thunk {
        // -- partie masquée ici --
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
