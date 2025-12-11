use rocket::http::Status;

pub mod ticket;
pub mod file_info;
pub mod titles;
pub mod upload;
pub mod download;

#[get("/v1/status")]
pub fn status_get() -> Status {
    Status::NoContent
}

#[head("/v1/status")]
pub fn status_head() -> Status {
    Status::NoContent
}