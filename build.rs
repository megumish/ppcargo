#![warn(missing_docs)]
//! build shell compeletion file
#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/cli.rs");

/// main routine
fn main() {
    let mut app = build_cli();
    //app.gen_completions("ppcargo", Shell::Bash, "./");
    app.gen_completions("ppcargo", Shell::Fish, "./");
    //app.gen_completions("ppcargo", Shell::Zsh, "./");
    //app.gen_completions("ppcargo", Shell::PowerShell, "./");
}

