#![warn(missing_docs)]
//! # sst
//!
//! A project for learning rust. This is a Star Trek game converted to rust.
//! It used **LOTS** of ideas from all sorts of samples, not just rust.
//! Also looked at C+, Java, Python, and even good old Basic (line numbers
//! were the best).
//!

//mod helpers;
mod astro;
mod ship_info;
mod manifest;
mod ui;
// todo: this
// research this stuff
// temp stuff
// bug: this is broken big time
// fixme: because I could be better
// optimize me for even better
// changed some stuff
// review this could be better
// debug this new code
// hack to get around this
// jap jap

fn main() {
    ui::logo_screen::game_logo();
    //astro::AstroUniverse::construct_galaxy((0, 0));

    ui::cmd_proc::command_processor();

    println!("Good Bye... Live long and Prosper");
}
