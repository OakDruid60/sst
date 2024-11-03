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

use rand::Rng;
macro_rules! nothing_jap {
    ($s1:expr, $s2:expr, $x1:expr) => {
        println!("this stuff is {}", $s1);
        let x: i32 = 10;
        for counter in 0..rand::thread_rng().gen_range(0..$x1) {
            println!(
                "that other stuff   s2=>{}<=    x1={}  counter={}     s1={}",
                $s2, $x1, counter, $s1
            );
        }
    };
}

fn main() {
    let a = "oakDruid60";
    nothing_jap!(a, "abc    kduw", 3);
    ui::logo_screen::game_logo();

    ui::cmd_proc::command_processor();

    println!("Good Bye... Live long and Prosper");
}
