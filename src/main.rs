#![allow(dead_code)]
mod cmd;
mod organizer;


use crate::organizer::Organizer;


#[allow(unused_assignments)]
fn main() {
    let mut organizer: Organizer = organizer::load_organizer();
    crate::cmd::execute(&mut organizer);
}