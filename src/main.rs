#![warn(missing_docs)]
//! tools for ppc(programming professional and coding)
#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

/// main routine
fn main() {
    let app = App::new(crate_name!())
        .setting(AppSettings::DeriveDisplayOrder)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());
}
