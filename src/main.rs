#[macro_use] extern crate clap;
extern crate users;

use clap::App;
use clap::AppSettings;

mod commands;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
    ;
    let matches = app.get_matches();

    match matches.subcommand() {
        ("hello", Some(matches)) => commands::hello::run(matches),
        _                        => unreachable!(),
    }
}
