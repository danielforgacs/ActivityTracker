use crate::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub port: u16,
    pub dbpath: std::path::PathBuf,
}

#[derive(Debug)]
struct ConfigBuilder {
    url: String,
    port: u16,
    dbpath: std::path::PathBuf,
}

impl ConfigBuilder {
    fn new() -> Self {
        let mut path = std::path::PathBuf::new();
        path.push("activitytracker_db.json");

        ConfigBuilder {
            url: ADDRESS.to_string(),
            port: 8000,
            dbpath: path,
        }
    }

    fn finish(&self) -> Result<Config, String> {
        if !self.dbpath.is_file() {
            File::create(&self.dbpath).unwrap().write(b"[]").unwrap();
        }
        let canon_path = self.dbpath
            .clone()
            .canonicalize()
            .map_err(|err| format!("Error getting canonicised path: {}", err))?;
        Ok(Config::new(self.url.clone(), self.port, canon_path))
    }

    fn url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    fn port(&mut self, port: u16) -> &mut Self{
        self.port = port;
        self
    }

    fn dbpath(&mut self, path: std::path::PathBuf) -> &mut Self {
        self.dbpath = path;
        self
    }
}

impl Config {
    fn new(
        url: String,
        port: u16,
        dbpath: std::path::PathBuf,
    ) -> Self {
        Self { url, port, dbpath }
    }
}

pub fn get_congig() -> Result<Config, String> {
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
                .help("File based database path."),
        ])
        .get_matches();
    let mut config = ConfigBuilder::new();
    if let Some(url) = matches.get_one::<String>("url") {
        config.url(url.clone());
    }
    if let Some(port) = matches.get_one::<u16>("port") {
        config.port(*port);
    }
    if let Some(path) = matches.get_one::<String>("dbfile") {
        config.dbpath(std::path::Path::new(path).to_path_buf());
    }
    let config = config.finish()?;
    dbg!(&config);
    Ok(config)
}
