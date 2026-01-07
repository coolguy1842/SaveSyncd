use std::{collections::HashMap, fs, path::Path, str::FromStr};

use fs_extra::dir::{self, get_dir_content2};
use rocket::{State, http::Status, serde::{Deserialize, json::Json}};
use serde::Serialize;
use uuid::Uuid;
use crate::{config::Config, v1::ticket::{Container, Ticket, TicketType, Tickets}, versions::v1::{file_info::{ClientFileInfo, DownloadAction, DownloadFileInfo, file_hash}, ticket::{copy_dir_all, ticket_path, tickets_path}}};

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginBody {
    id: u64,
    container: String,
    existing_files: Vec<ClientFileInfo>
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct BeginResponse {
    ticket: String,
    files: Vec<DownloadFileInfo>
}

#[post("/v1/download/begin", format = "application/json", data = "<data>")]
pub fn download_begin(tickets: &State<Tickets>, config: &State<Config>, data: Json<BeginBody>) -> Result<Json<BeginResponse>, Status> {
    let container = Container::from_str(&data.container).map_err(|_| Status::BadRequest)?;
    let mut ticket_map = tickets.lock().map_err(|_| Status::InternalServerError)?;

    let title_path = config.data_directory().join(format!("{:X}", data.id));
    let container_path = title_path.join(container.to_string().to_lowercase());
    let container_path_str = container_path.to_str().expect("Failed to get string of container path");

    if !container_path.exists() {
        return Err(Status::NoContent)
    }
    
    let ticket_id = Uuid::new_v4();
    let ticket = Ticket { id: ticket_id, title_id: data.id, kind: TicketType::DOWNLOAD, container: container };

    let base_staging_path = tickets_path();
    let staging_path = ticket_path(ticket_id);

    if !base_staging_path.exists() && fs::create_dir_all(&base_staging_path).is_err() {
        return Err(Status::InternalServerError)
    }

    copy_dir_all(&container_path, &staging_path).expect("Failed to copy container path to staging path");
    ticket_map.insert(ticket_id, ticket);
    drop(ticket_map);

    let mut actions: HashMap<String, DownloadFileInfo> = data.existing_files
        .iter()
        .map(|f| ( f.path.clone(), DownloadFileInfo{ action: DownloadAction::REMOVE, path: f.path.clone(), hash: f.hash.clone(), size: Some(f.size) } ))
        .collect();

    let contents = get_dir_content2(&container_path, &dir::DirOptions::new()).expect("Failed to get staging path contents");
    for path in contents.files {
        let Some(file) = path.strip_prefix(container_path_str) else { continue; };
        let Ok(metadata) = fs::metadata(&path) else { continue; };

        let size = metadata.len();
        let Ok(hash) = file_hash(Path::new(&path)) else { continue; };

        if let Some(info) = actions.get_mut(file) {
            if info.size == Some(size) && info.hash == Some(hash.clone()) {
                info.action = DownloadAction::KEEP;
                continue;
            }

            info.action = DownloadAction::REPLACE;
            info.size = Some(size);
            info.hash = Some(hash);

            continue;
        }

        actions.insert(file.to_string(), DownloadFileInfo {
            path: file.to_string(),
            size: Some(size),
            hash: Some(hash),
            action: DownloadAction::CREATE
        });
    }

    if actions.iter().all(|f| f.1.action == DownloadAction::KEEP) {
        return Err(Status::NoContent)
    }

    Ok(Json(BeginResponse{ ticket: ticket_id.hyphenated().to_string(), files: actions.iter().map(|action| action.1.clone()).collect() }))
}
