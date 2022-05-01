use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use crate::db::record::{all, Record};

pub fn routes() -> Vec<rocket::Route> {
  routes![get_all]
}

#[get("/")]
pub fn get_all() -> Result<Json<Vec<Record>>, Custom<String>> {
  match all() {
    Ok(records) => Ok(Json(records)),
    Err(err) => Err(Custom(Status::InternalServerError, err)),
  }
}
