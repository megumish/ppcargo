#[warn(missing_docs)]
use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

/// Build clap arg parser
pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
}
