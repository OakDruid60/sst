//! # sst
//!
//! A project for learning rust. This is a Star Trek game converted to rust.
//! It used **LOTS** of ideas from all sorts of samples, not just rust.
//! Also looked at C+, Java, Python, and even good old Basic (line numbers
//! were the best).
//!

//mod command;
//mod constants;
//mod disp;
//mod enums;
//mod gamedata;
mod helpers;
//mod movement;
//mod ship;
//mod statistics;
//mod ui::logo;
//mod ui::sensor;
mod enterprise;
mod manifest;
mod ui;
//mod weapon;

fn main() {
    ui::logo::game_logo();

    ui::command_processor();

    println!("Good Bye");
}
