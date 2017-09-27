#![warn(missing_docs)]
//! tools for ppc(programming professional and coding)
#[macro_use]
extern crate clap;

mod cli;

/// main routine
fn main() {
    let matches = cli::build_cli().get_matches();
}
