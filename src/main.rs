use std::{collections::HashMap, sync::{Arc, Mutex}};

use rocket::{data::{Limits, ToByteUnit}, tokio};
use crate::{config::Config, versions::v1};

#[macro_use] extern crate rocket;

pub mod config;
pub mod versions;

#[cfg(feature = "tray")]
pub mod tray_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let rocket = rocket::custom(figment)
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
        ]).ignite().await?;

    let _shutdown = rocket.shutdown();
    let rocket_handle = tokio::spawn(async move {
        rocket.launch().await
    });

    #[cfg(feature = "tray")]
    {
        use crate::tray_app::Application;
        let app = Application::new();

        app.run();
        _shutdown.notify();
        let _ = tokio::join!(rocket_handle);
    }

    #[cfg(not(feature = "tray"))]
    {
        let _ = tokio::join!(rocket_handle);
    }

    v1::ticket::clear_tickets_path().expect("Failed to cleanup tickets path");

    Ok(())
}
