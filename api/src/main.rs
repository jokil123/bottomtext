#[macro_use]
extern crate rocket;

mod db;

// #[get("/api/get-frames")]
// fn get_frames() -> Json {
//     todo!()
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
    // .mount("/", routes![get_frames])
}
