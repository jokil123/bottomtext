use rocket::http::Status;
use rocket::serde::json::Json;

use crate::db::types::FramesJson;

#[macro_use]
extern crate rocket;

mod db;

#[get("/api/frames")]
fn frames() -> Result<Json<FramesJson>, Status> {
    let frames = match db::read_frames() {
        Ok(f) => f,
        Err(_e) => return Err(Status::InternalServerError),
    };

    Ok(Json(frames))
}

#[post("/api/frame", data = "<frame>")]
fn add_frame(frame: Json<db::types::FrameJson>) -> Result<(), Status> {
    match db::write_frame(frame.into_inner()) {
        Ok(_) => Ok(()),
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![frames, add_frame])
}
