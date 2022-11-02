use crate::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub port: u16,
    pub dbpath: std::path::PathBuf,
}

impl Config {
    fn new() -> Self {
        let matches = clap::Command::new("activitytracker")
            .version(env!("CARGO_PKG_VERSION"))
            .about(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/README.md"
            )))
            .args([
                clap::Arg::new("url")
                    .short('u')
                    .long("url")
                    .value_name("URL")
                    .default_value(ADDRESS)
                    .help("Set the url to serve."),
                clap::Arg::new("port")
                    .short('p')
                    .long("port")
                    .value_name("PORT")
                    .value_parser(clap::value_parser!(u16).range(3000..))
                    .default_value(PORT)
                    .help("Set the localhost port to serve."),
                clap::Arg::new("dbfile")
                    .short('d')
                    .long("dbfile")
                    .help("File based database path.")
            ])
            .get_matches();
        let url = matches.get_one::<String>("url").unwrap().to_owned();
        let port = *matches.get_one::<u16>("port").unwrap();
        let dbfile = match matches.get_one::<String>("dbfile") {
            Some(dbfile) => dbfile.to_string(),
            Option::None => {
                "activitytracker_db.json".to_string()
            }
        };
        let mut dbpath = std::path::PathBuf::new();
        dbpath.push(dbfile);
        std::fs::File::create(&dbpath)
            .unwrap()
            .write_all(b"[]")
            .unwrap();
        Config { url, port, dbpath }
    }
}

pub fn get_congig() -> Config {
    Config::new()
}
