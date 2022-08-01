pub const ADDRESS: &str = "127.0.0.1";
pub const PORT: &str = "8000";

pub struct Config {
    pub url: String,
    pub port: u16,
}

impl Config {
    fn new() -> Self {
        let matches = clap::Command::new("timetracker")
            .version(env!("CARGO_PKG_VERSION"))
            .about(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md")))
            .arg(
                clap::Arg::new("url")
                    .short('u')
                    .long("url")
                    .value_name("URL")
                    .default_value(ADDRESS)
                    .help("Set the url to serve.")
            )
            .arg(
                clap::Arg::new("port")
                    .short('p')
                    .long("port")
                    .value_name("PORT")
                    .value_parser(clap::value_parser!(u16).range(3000..))
                    .default_value(PORT)
                    .help("Set the localhost port to serve.")
            )
            .get_matches();
        let url = matches
            .value_of("url")
            .unwrap()
            .to_string();
        let port: u16 = *matches
            .get_one("port")
            .unwrap();
        Config {
            url,
            port,
        }
    }
}

pub fn get_congig() -> Config {
    Config::new()
}
