#![allow(dead_code)]
use lib::organizer::{self, Organizer};

mod lib;

#[allow(unused_assignments)]
fn main() {
    let mut organizer: Organizer = organizer::load_organizer();
    lib::cmd::execute(&mut organizer);
}