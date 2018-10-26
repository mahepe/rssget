extern crate clap;
use clap::{App, Arg, SubCommand};

mod alias;
mod fetch;
mod read;
mod utils;

fn main() {
    let matches = App::new("rssget")
        .version("0.1.0")
        .about("A minimal RSS reader.")
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
        ).subcommand(
            SubCommand::with_name("read")
                .about("read the fetched stream")
                .arg(
                    Arg::with_name("attrs")
                        .help("a list of attributes to read")
                        .required(false)
                        .min_values(1),
                ).arg(
                    Arg::with_name("feed")
                        .help("read from a specific feed")
                        .required(false)
                        .short("f")
                        .long("feed")
                        .default_value(""),
                ),
        ).subcommand(
            SubCommand::with_name("alias")
                .about("give an alias to a feed")
                .arg(
                    Arg::with_name("feed_alias")
                        .index(1)
                        .required(true)
                        .help("feed alias")
                        .takes_value(true),
                ).arg(
                    Arg::with_name("feed_url")
                        .index(2)
                        .required(true)
                        .help("feed url")
                        .takes_value(true),
                ),
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("fetch") {
        if let Some(feed) = matches.value_of("feed") {
            match fetch::fetch(
                feed,
                "asd.dat".to_string(),
                "aux.dat".to_string(),
                "alias.dat".to_string(),
            ) {
                Ok(()) => (),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("read") {
        let mut attrs: Vec<&str>;
        if let Some(att) = matches.values_of("attrs") {
            attrs = att.collect();
        } else {
            attrs = vec!["title", "link"]
        }
        match read::read(
            "asd.dat".to_string(),
            "aux.dat".to_string(),
            "alias.dat".to_string(),
            attrs,
            matches.value_of("feed").unwrap().to_string(),
        ) {
            Ok(()) => (),
            Err(e) => println!("{:?}", e),
        }
    }

    if let Some(matches) = matches.subcommand_matches("alias") {
        if let (Some(feed_alias), Some(feed_url)) =
            (matches.value_of("feed_alias"), matches.value_of("feed_url"))
        {
            match alias::alias(
                feed_alias.to_string(),
                feed_url.to_string(),
                "alias.dat".to_string(),
            ) {
                Ok(()) => (),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
