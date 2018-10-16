extern crate clap;
use clap::{App, Arg, SubCommand};

mod fetch;
mod utils;

fn main() {
    let matches = App::new("rssget")
        .version("0.1.0")
        .about("A minimal RSS client.")
        .subcommand(
            SubCommand::with_name("fetch")
                .about("fetch an RSS stream from the given feed")
                .arg(
                    Arg::with_name("feed")
                        .index(1)
                        .required(true)
                        .help("feed URL or alias")
                        .takes_value(true),
                ),
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("fetch") {
        if let Some(feed) = matches.value_of("feed") {
            fetch::fetch(feed, "asd.dat".to_string());
        }
    }
}
