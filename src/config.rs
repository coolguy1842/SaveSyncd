use std::{fs::{self, File}, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing, skip_deserializing)]
    config_path: PathBuf,

    port: u16,
    data_directory: PathBuf
}

impl Config {
    fn config_path() -> PathBuf {
        dirs::config_dir().expect("Failed to get config dir").join("SaveSyncd")
    }

    fn default_config_file() -> PathBuf {
        Config::config_path().join("config.json")
    }

    // use none for default path
    pub fn load(path: Option<PathBuf>) -> Self {
        let config = Config {
            config_path: path.unwrap_or(Config::default_config_file()),

            port: 8000,
            data_directory: dirs::data_dir().expect("Failed to get data dir").join("SaveSyncd")
        };

        if !fs::exists(&config.config_path).unwrap_or(false) {
            config.save();
            return config
        }

        // panic if invalid config
        serde_json::from_reader(BufReader::new(File::open(&config.config_path).expect("Failed to open file"))).expect("ERROR: Invalid Config")
    }

    fn save(&self) {
        if self.config_path != Config::default_config_file() {
            return;
        }
        
        if !fs::exists(&self.config_path).unwrap_or(false) {
            if fs::create_dir_all(&self.config_path).is_err() {
                return;
            }
        }

        fs::write(&self.config_path, serde_json::to_string_pretty(self).expect("Failed to stringify config")).expect("Failed to write config");
    }

    pub fn port(&self) -> u16 { return self.port }
    pub fn data_directory(&self) -> PathBuf { return self.data_directory.clone() }
}