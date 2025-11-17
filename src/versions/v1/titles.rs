use std::{collections::HashMap, fs::File, io::{self, Read}, path::Path};

use fs_extra::dir::{DirOptions, get_dir_content, get_dir_content2};
use rocket::State;
use serde::Serialize;

use crate::{config::Config, versions::v1::file_info::ServerFileInfo};

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
            let path = file.strip_prefix(&dir);
            if path.is_none() {
                continue;
            }
            
            let file = File::open(&file).expect("Failed to open file");
            let metadata = file.metadata().expect("Failed to get file metadata");
            
            let mut context = md5::Context::new();
            let mut buffer = [0; 4096];
            let mut reader = io::BufReader::new(file);

            loop {
                let bytes_read = reader.read(&mut buffer).expect("Failed to get bytes from file");
                if bytes_read == 0 {
                    break;
                }

                context.consume(&buffer[..bytes_read]);
            }

            out.push(ServerFileInfo { path: path.unwrap().to_string(), size: metadata.len(), hash: format!("{:x}", context.finalize()) });
        }
    }

    out
}

#[get("/v1/titles")]
pub async fn titles(config: &State<Config>) -> String {
    let mut out: TitlesResponse = HashMap::new();

    let data_dir = config.data_directory();
    let data_dir_str = format!("{}/", data_dir.to_str().expect("Failed to get string of data directory"));
    for directory in get_dir_content2(data_dir, &DirOptions { depth: 1 }).expect("Failed to get title data diorectory content").directories {
        let id = u64::from_str_radix(directory.strip_prefix(&data_dir_str).unwrap_or(""), 16);
        if id.is_err() {
            continue;
        }

        let info = TitleInfo { save: get_dir_info(format!("{}/save/", directory)), extdata: get_dir_info(format!("{}/extdata/", directory)) };
        if !info.save.is_empty() || !info.extdata.is_empty() {
            out.insert(id.unwrap(), info);
        }
    }

    return serde_json::to_string_pretty(&out).expect("Failed to stringify titles");
}
