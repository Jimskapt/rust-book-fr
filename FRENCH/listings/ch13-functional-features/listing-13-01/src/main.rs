// ANCHOR: here
use std::thread;
use std::time::Duration;

fn simuler_gros_calcul(intensite: u32) -> u32 {
    println!("calcul très lent ...");
    thread::sleep(Duration::from_secs(2));
    intensite
}
// ANCHOR_END: here

fn main() {}
