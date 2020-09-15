fn main() {
    // ANCHOR: here
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("salut"));

    fn prend_un_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // -- partie masquée ici --
    }

    fn retourne_un_long_type() -> Box<dyn Fn() + Send + 'static> {
        // -- partie masquée ici --
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
