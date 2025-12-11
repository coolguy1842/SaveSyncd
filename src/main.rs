use std::{collections::HashMap, sync::{Arc, Mutex}};

use rocket::data::{Limits, ToByteUnit};

use crate::{config::Config, versions::v1};

#[macro_use] extern crate rocket;

pub mod config;
pub mod versions;

#[main]
async fn main() -> Result<(), rocket::error::Error> {
    // cleanup previous if exists
    v1::ticket::clear_tickets_path().expect("Failed to clear old tickets path");
    let tickets: v1::ticket::Tickets = Arc::new(Mutex::new(HashMap::new()));
    let config = Config::load();
    
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", config.port()))
        .merge(("limits", Limits::new()
            .limit("bytes", 100.mebibytes())
            .limit("json", 8.mebibytes())
        ));

    rocket::custom(figment)
        .manage(tickets)
        .manage(config)
        .mount("/", routes![
            v1::status_get,
            v1::status_head,
            v1::titles::titles,

            v1::upload::begin::upload_begin,
            v1::upload::file::upload_file,
            v1::upload::end::upload_end,
            v1::upload::cancel::upload_cancel,

            v1::download::begin::download_begin,
            v1::download::file::download_file,
            v1::download::end::download_end
        ])
        .launch()
        .await?;

    v1::ticket::clear_tickets_path().expect("Failed to cleanup tickets path");
    Ok(())
}
