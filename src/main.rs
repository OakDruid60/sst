#![warn(missing_docs)]
//! # sst
//!
//! A project for learning rust. This is a Star Trek game converted to rust.
//! It used **LOTS** of ideas from all sorts of samples, not just rust.
//! Also looked at C+, Java, Python, and even good old Basic (line numbers
//! were the best).
//!

mod astro;
mod constants;
mod manifest;
mod ship_info;
mod ui;

mod test_complete_game;

fn main() {

    ui::logo_screen::game_logo();
    
    let st_cmds_vec: Vec<String> = vec!();
 
    ui::cmd_proc::command_processor(&st_cmds_vec);

    println!("Good Bye... Live long and Prosper");
}
