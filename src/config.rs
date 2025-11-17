use std::{fs::{self, File}, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    port: u16,
    data_directory: PathBuf
}

impl Config {
    fn config_path() -> PathBuf {
        dirs::config_dir().expect("Failed to get config dir").join("SaveSyncd")
    }

    fn config_file() -> PathBuf {
        Config::config_path().join("config.json")
    }

    pub fn load() -> Self {
        let config = Config { port: 8000, data_directory: dirs::data_dir().expect("Failed to get data dir").join("SaveSyncd") };
        let path = Config::config_file();

        if !fs::exists(path.clone()).unwrap_or(false) {
            config.save();
            return config
        }

        let res = serde_json::from_reader(BufReader::new(File::open(path).expect("Failed to open file")));
        if res.is_err() {
            config.save();
            return config
        }

        res.unwrap()
    }

    pub fn save(&self) {
        if !fs::exists(Config::config_path()).unwrap_or(false) {
            if fs::create_dir_all(Config::config_path()).is_err() {
                return;
            }
        }

        fs::write(Config::config_file(), serde_json::to_string_pretty(self).expect("Failed to stringify config")).expect("Failed to write config");
    }

    pub fn port(&self) -> u16 { return self.port }
    pub fn data_directory(&self) -> PathBuf { return self.data_directory.clone() }
}