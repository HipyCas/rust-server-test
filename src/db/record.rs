use serde::{Deserialize, Serialize};

use crate::db::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
  day: String,
  id: Id,
  inside_temp: f64,
  inside_hum: f64,
  outside_temp: f64,
  outside_hum: f64,
}

fn load() -> Result<Vec<Record>, String> {
  let base_path = std::path::PathBuf::from(file!());
  let root_path = std::path::PathBuf::from("/"); // TODO It is totally safe to remove this
  let path = base_path
    .parent()
    .unwrap_or(&root_path)
    .join("records.json");
  let contents = match std::fs::read_to_string(path) {
    Ok(con) => con,
    Err(e) => return Err(e.to_string()),
  };
  match serde_json::from_str::<Vec<Record>>(&contents) {
    Ok(res) => Ok(res),
    Err(e) => Err(e.to_string()),
  }
}

pub fn all() -> Result<Vec<Record>, String> {
  load()
}
