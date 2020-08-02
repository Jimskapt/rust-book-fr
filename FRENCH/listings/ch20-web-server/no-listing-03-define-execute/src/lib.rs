pub struct GroupeTaches;

// ANCHOR: here
impl GroupeTaches {
    // -- partie masquée ici--
    // ANCHOR_END: here
    pub fn new(size: usize) -> GroupeTaches {
        GroupeTaches
    }

    // ANCHOR: here
    pub fn executer<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
// ANCHOR_END: here

fn main() {}
