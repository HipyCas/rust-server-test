use rocket::response::Debug;
use rocket::serde::json::Json;

use crate::db::device::{all, save, Device};
use crate::db::Id;

pub fn routes() -> Vec<rocket::Route> {
  routes![get_all, delete, get, update, create, get_owned]
}

#[get("/")]
pub fn get_all() -> Result<Json<Vec<Device>>, Debug<String>> {
  //? Change by custom so caller gets some feedback?
  match all() {
    Ok(devices) => Ok(Json(devices)),
    Err(err) => Err(Debug::from(err)),
  }
}

#[delete("/<id>")]
pub fn delete(id: Id) -> Result<Json<Device>, Debug<String>> {
  let res = all().map_err(Debug::from)?;
  let new: Vec<&Device> = res.iter().filter(|device| device.id != id).collect();
  save(new).map_err(Debug::from)?;
  Ok(Json(
    res
      .into_iter()
      .find(|device| device.id == id)
      .ok_or(Debug::from("Device not found".to_owned()))?,
  ))
}

#[get("/<id>")]
pub fn get(id: Id) -> Result<Option<Json<Device>>, Debug<String>> {
  match all()
    .map_err(Debug::from)?
    .into_iter()
    .find(|device| device.id == id)
  {
    Some(item) => Ok(Some(Json(item))),
    None => Ok(None),
  }
}

#[put("/<id>", data = "<device>")]
pub fn update(id: Id, device: Json<Device>) -> Result<Json<Device>, Debug<String>> {
  // TODO Return updated device
  save(
    all()
      .map_err(Debug::from)?
      .iter()
      .filter(|d| d.id != id)
      .chain(vec![&device.0])
      .collect::<Vec<&Device>>(),
  )
  .map_err(Debug::from)?;
  Ok(device)
}

#[post("/", data = "<device>")]
pub fn create(device: Json<Device>) -> Result<Json<Device>, Debug<String>> {
  save(
    all()
      .map_err(Debug::from)?
      .iter()
      .chain(vec![&device.0])
      .collect::<Vec<&Device>>(),
  )
  .map_err(Debug::from)?;
  Ok(device) // Returning the device gives me ownership problems
}

#[get("/owned/<user_id>")]
pub fn get_owned(user_id: Id) -> Result<Json<Vec<Device>>, Debug<String>> {
  Ok(Json(
    all()
      .map_err(Debug::from)?
      .into_iter()
      .filter(|device| device.permissions.ownerId == user_id)
      .collect::<Vec<Device>>(),
  ))
}
