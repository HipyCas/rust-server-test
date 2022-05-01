// use serde::{Deserialize, Serialize};

pub mod device;
pub mod record;

pub type Id = u32;

// pub struct JsonStore<T: Serialize + Deserialize<'static>> {
//   data: T,
// }

// impl<T: Serialize + Deserialize<'static>> JsonStore<T> {
//   pub fn from_file_array(file: std::path::PathBuf) -> Result<Vec<T>, String> {
//     let contents = match std::fs::read_to_string(file) {
//       Ok(con) => con,
//       Err(e) => return Err(e.to_string()),
//     };
//     println!("{:#?}", contents);
//     match serde_json::from_str::<Vec<T>>(&'static contents) {
//       Ok(res) => Ok(res),
//       Err(e) => Err(e.to_string()),
//     }
//   }
// }
