// https://github.com/rust-lang/rust/issues/46379
// IDE incorrectly generates warning for unused code (since not called from `main`)
#![allow(non_snake_case, dead_code)]
#[macro_use]

mod structs;
mod tests;
mod handler;
mod DimensionsHelper;

fn main() {
    println!("Hello, world!");
}