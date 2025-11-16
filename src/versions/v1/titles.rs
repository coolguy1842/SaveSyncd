use std::collections::HashMap;

use rocket::{State, tokio::fs::read_dir};
use serde::Serialize;

use crate::{config::Config, versions::v1::file_info::ServerFileInfo};

#[derive(Serialize)]
struct TitleInfo {
    save: Vec<ServerFileInfo>,
    extdata: Vec<ServerFileInfo>
}

type TitlesResponse = HashMap<u64, TitleInfo>;

#[get("/v1/titles")]
pub async fn titles(config: &State<Config>) -> String {
    let out: TitlesResponse = HashMap::new();
    let mut reader = read_dir(config.data_directory()).await.expect("Failed to read titles directory");

    loop {
        if let Some(f) = reader.next_entry().await.unwrap() {
            let id = u64::from_str_radix(f.file_name().to_str().unwrap(), 16);
            if id.is_err() || !f.file_type().await.unwrap().is_dir() {
                continue;
            }

            
            println!("{:X}", id.unwrap());
        }
        else {
            break;
        }
    } 

    return serde_json::to_string(&out).expect("Failed to stringify titles");
}
