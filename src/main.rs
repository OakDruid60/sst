#![warn(missing_docs)]
//! # sst
//!
//! A project for learning rust. This is a Star Trek game converted to rust.
//! It used **LOTS** of ideas from all sorts of samples, not just rust.
//! Also looked at C+, Java, Python, and even good old Basic (line numbers
//! were the best).
//!
/*
use std::fs::File;
use std::io::{Read, Write};
use serde::Serialize;
use toml::to_string;
*/
mod astro;
mod constants;
mod manifest;
mod ship_info;
mod ui;

//mod test_complete_game;

fn main() {

    ui::logo_screen::game_logo();
    
    let st_cmds_vec: Vec<String> = vec!();
 
    ui::cmd_proc::command_processor(&st_cmds_vec);

    println!("Good Bye... Live long and Prosper");
}
/*    
#[derive(Serialize)]
struct TestCmds {
    test_cmds_vec: Vec<String>,
}

fn write_cmds_to_file(config: &TestCmds, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
 
fn write_toml(cmds_vec:Vec<String>) {
    let config = TestCmds {
        test_cmds_vec: cmds_vec,
    };

    if let Err(e) = write_cmds_to_file(&config, "test_data/test_01_cmds.toml") {
        eprintln!("Error: {}", e);
    } else {
        println!("Config file created successfully.");
    }
}

fn read_toml() -> String {
    let mut file = File::open("test_data/test_01_cmds.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    contents
    // At this point, `contents` contains the content of the TOML file
    //println!("{}", contents);
}
    */