use crate::db::Id;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
  // TODO Make some fields like maybe permissions optional
  pub id: Id,
  pub name: String,
  pub location: String,
  pub status: DeviceStatus,
  pub permissions: DevicePermissions,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum DeviceStatus {
  Online = 0,
  Warning = 1,
  Error = 2,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DevicePermissions {
  pub url: UrlShare,
  pub people: Vec<UserShare>,
  pub ownerId: Id,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlShare {
  pub enabled: bool,
  pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserShare {
  pub userId: Id,
  pub role: UserShareRole, // TODO Some kind of type checking, enum or something?
}

// TODO see https://serde.rs/enum-number.html
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum UserShareRole {
  view,
  edit,
}

// impl rocket::data::FromData for Device {
//   type Error = String;
//   fn from_data(rocket::Request, data: rocket::Data) {

//   }
// }

fn load() -> Result<Vec<Device>, String> {
  let base_path = std::path::PathBuf::from(file!());
  let root_path = std::path::PathBuf::from("/"); // TODO It is totally safe to remove this
  let path = base_path
    .parent()
    .unwrap_or(&root_path)
    .join("devices.json");
  let contents = match std::fs::read_to_string(path) {
    Ok(con) => con,
    Err(e) => return Err(e.to_string()),
  };
  match serde_json::from_str::<Vec<Device>>(&contents) {
    Ok(res) => Ok(res),
    Err(e) => Err(e.to_string()),
  }
}

pub fn all() -> Result<Vec<Device>, String> {
  load()
}

pub fn save(data: Vec<&Device>) -> Result<(), String> {
  let base_path = std::path::PathBuf::from(file!());
  let root_path = std::path::PathBuf::from("/"); // TODO It is totally safe to remove this
  let path = base_path
    .parent()
    .unwrap_or(&root_path)
    .join("devices.json");
  std::fs::remove_file(&path).map_err(|e| e.to_string())?;
  std::fs::File::create(&path)
    .map_err(|e| e.to_string())?
    .write_all(
      serde_json::to_string_pretty(&data)
        .map_err(|e| e.to_string())?
        .as_bytes(),
    )
    .map_err(|e| e.to_string())?;
  Ok(())
}

#[allow(dead_code)]
pub fn save_owned(data: Vec<Device>) -> Result<(), String> {
  // Can probably get rid of
  let base_path = std::path::PathBuf::from(file!());
  let root_path = std::path::PathBuf::from("/"); // TODO It is totally safe to remove this
  let path = base_path
    .parent()
    .unwrap_or(&root_path)
    .join("devices.json");
  std::fs::remove_file(&path).map_err(|e| e.to_string())?;
  std::fs::File::create(&path)
    .map_err(|e| e.to_string())?
    .write_all(
      serde_json::to_string_pretty(&data)
        .map_err(|e| e.to_string())?
        .as_bytes(),
    )
    .map_err(|e| e.to_string())?;
  Ok(())
}
