use std::io::prelude::*;
use std::fs::File;
use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    database_url: String,
}

impl Config {
    pub fn init_config_from_file(fp: &str) -> Result<Config, String> {
        let mut f = try!(File::open(fp).map_err(|e| e.to_string()));
        let mut s = String::new();
        try!(f.read_to_string(&mut s).map_err(|e| e.to_string()));
        let config: Config = try!(serde_yaml::from_str(&s).map_err(|e| e.to_string()));
        Ok(config)
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
