#[macro_use]
extern crate rocket;

use rocket::response::status::NoContent;

#[get("/health_check")]
fn index() -> NoContent {
    NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
