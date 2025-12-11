use std::{collections::HashMap, fs, path::Path};

use fs_extra::dir::{DirOptions, get_dir_content, get_dir_content2};
use rocket::State;
use serde::Serialize;

use crate::{config::Config, versions::v1::file_info::{ServerFileInfo, file_hash}};

#[derive(Serialize)]
struct TitleInfo {
    save: Vec<ServerFileInfo>,
    extdata: Vec<ServerFileInfo>
}

type TitlesResponse = HashMap<u64, TitleInfo>;

fn get_dir_info(dir: String) -> Vec<ServerFileInfo> {
    let path = Path::new(&dir).to_path_buf();
    let mut out: Vec<ServerFileInfo> = Vec::new();

    if path.try_exists().unwrap_or(false) {
        for file in get_dir_content(path).expect("Failed to get title save content").files {
            let Some(path) = file.strip_prefix(&dir) else { continue; };
            let Ok(metadata) = fs::metadata(&file) else { continue; };
            let Ok(hash) = file_hash(Path::new(&file)) else { continue; };

            out.push(ServerFileInfo { path: path.to_string(), size: metadata.len(), hash });
        }
    }

    out
}

#[get("/v1/titles")]
pub async fn titles(config: &State<Config>) -> String {
    let mut out: TitlesResponse = HashMap::new();

    let data_dir = config.data_directory();
    let data_dir_str = format!("{}/", data_dir.to_str().expect("Failed to get string of data directory"));

    let Ok(contents) = get_dir_content2(data_dir, &DirOptions { depth: 1 }) else { return "{}".to_string() };
    for directory in contents.directories {
        let Ok(id) = u64::from_str_radix(directory.strip_prefix(&data_dir_str).unwrap_or(""), 16) else { continue; };
        
        let info = TitleInfo { save: get_dir_info(format!("{}/save/", directory)), extdata: get_dir_info(format!("{}/extdata/", directory)) };
        if !info.save.is_empty() || !info.extdata.is_empty() {
            out.insert(id, info);
        }
    }

    return serde_json::to_string_pretty(&out).expect("Failed to stringify titles");
}