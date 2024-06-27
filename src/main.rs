#![allow(unused)]

use std::fs;
use std::io::{stdout, stdin, Write};
use std::path::Path;
use serialization::*;
use data::*;
//use fileio::*;

mod fileio;
mod serialization;
mod data;
mod ui;

fn main() {
    ui::main_loop();
}