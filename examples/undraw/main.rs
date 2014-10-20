/// This file is distributed with boxdraw-rs as an example usage of
/// that crate.
///
/// However, note that the file can also be easily packaged up as its
/// own cargo package, with boxdraw as a dependency.
///
/// To set that up, just create the cargo package (via `cargo new`), and
/// then add the following to the `Cargo.toml` file:
///
/// ```
/// [dependencies.boxdraw-rs]
/// git = "https://github.com/pnkfelix/boxdraw-rs"
/// ```

extern crate "boxdraw-rs" as boxdraw;

use boxdraw::{Undraw, Script};

pub struct SimpleSearch;

impl Undraw for SimpleSearch {
    fn undraw(&self, picture: &str) -> Script {
        let _ = picture;
        unimplemented!()
    }
}

fn main() {
    check_simple()
}

fn check_simple() {
    boxdraw::check_undraw("...\n", &SimpleSearch).unwrap();

    boxdraw::check_undraw(".....\n\
                           .+-+.\n\
                           .|b|.\n\
                           .+-+.\n\
                          ",
                          &SimpleSearch).unwrap();

    boxdraw::check_undraw(".............\n\
                           .......+--+..\n\
                           .+-+...|cc|..\n\
                           .|b|...|cc|..\n\
                           .+-+...+--+..\n\
                           .............\n\
                          ",
                          &SimpleSearch).unwrap();

    boxdraw::check_undraw(".........\n\
                           .........\n\
                           .+-+.....\n\
                           .|b|.....\n\
                           .+-+.....\n\
                           .+--+....\n\
                           .|cc|....\n\
                           .|cc|....\n\
                           .+--+....\n\
                           .........\n\
                          ",
                          &SimpleSearch).unwrap();
}

#[test]
fn run_check_simple() { check_simple() }
