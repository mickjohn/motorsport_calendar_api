use std::io::prelude::*;
use std::fs::File;
use serde_yaml;

pub const DEFAULT_ENABLE_DATA_REFRESH: bool = true;
pub const DEFAULT_DATA_REFRESH_INTERVAL_SECONDS: u64 = 60*5;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    data_paths: Vec<String>,
    enable_data_refresh: Option<bool>,
    data_refresh_interval_seconds: Option<u64>,
}

impl Config {
    pub fn init_config_from_file(fp: &str) -> Result<Config, String> {
        let mut f = try!(File::open(fp).map_err(|e| e.to_string()));
        let mut s = String::new();
        try!(f.read_to_string(&mut s).map_err(|e| e.to_string()));
        let config: Config = try!(serde_yaml::from_str(&s).map_err(|e| e.to_string()));
        Ok(config)
    }

    pub fn data_paths(&self) -> &[String] { 
        &self.data_paths 
    }

    pub fn enable_data_refresh(&self) -> bool {
        self.enable_data_refresh.unwrap_or(DEFAULT_ENABLE_DATA_REFRESH)
    }

    pub fn data_refresh_interval_seconds(&self) -> u64 {
        self.data_refresh_interval_seconds
            .unwrap_or(DEFAULT_DATA_REFRESH_INTERVAL_SECONDS)
    }
}
