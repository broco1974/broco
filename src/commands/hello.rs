use clap::ArgMatches;
use users::{get_user_by_uid, get_current_uid};

fn username() -> String {
    let user = get_user_by_uid(get_current_uid()).unwrap();

    user.name().to_owned()
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("user", Some(_))  => println!("Hello, {}!", username()),
        ("world", Some(_)) => println!("Hello, world!"),
        ("", None)         => println!("Hello"),
        _                  => unreachable!(),
    }
}
