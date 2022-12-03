use crate::prelude::*;

#[derive(Debug)]
pub struct Config {
    url: String,
    port: u16,
    dbpath: path::PathBuf,
}

#[derive(Debug)]
struct ConfigBuilder {
    url: String,
    port: u16,
    dbpath: path::PathBuf,
}

impl Config {
    fn new(url: String, port: u16, dbpath: path::PathBuf) -> Self {
        Self { url, port, dbpath }
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn get_port(&self) -> &u16 {
        &self.port
    }

    pub fn get_url_w_port(&self) -> String {
        format!("{}:{}", self.get_url(), self.get_port())
    }

    pub fn get_dbpath(&self) -> &path::PathBuf {
        &self.dbpath
    }
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            url: "".to_string(),
            port: 0,
            dbpath: path::Path::new("activitytracker_db.json").to_path_buf(),
        }
    }

    fn finish(&self) -> Result<Config, String> {
        if !self.dbpath.is_file() {
            let byte_count = File::create(&self.dbpath).unwrap().write(b"[]").unwrap();
            if byte_count != b"[]".len() {
                return Err("Could not write initial database.".to_string());
            }
        }
        let canon_path = self
            .dbpath
            .clone()
            .canonicalize()
            .map_err(|err| format!("Error getting canonicised path: {}", err))?;
        Ok(Config::new(self.url.clone(), self.port, canon_path))
    }

    fn url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    fn dbpath(&mut self, path: path::PathBuf) -> &mut Self {
        self.dbpath = path;
        self
    }
}

pub fn get_congig() -> Result<Config, String> {
    let matches = clap::Command::new("activitytracker")
        .version(env!("CARGO_PKG_VERSION"))
        .about(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/about.txt"
        )))
        .args([
            clap::Arg::new("url")
                .short('u')
                .long("url")
                .default_value("127.0.0.1")
                .help("Set the url to serve."),
            clap::Arg::new("port")
                .short('p')
                .long("port")
                .value_parser(clap::value_parser!(u16).range(3000..))
                .default_value("8000")
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
        config.dbpath(path::Path::new(path).to_path_buf());
    }
    let config = config.finish()?;
    Ok(config)
}
