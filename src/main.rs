#[macro_use]
extern crate rocket;
// #[macro_use]
// extern crate log;

use rocket::response::content::Html;
use rocket::shield::Shield;
use rocket::Rocket;

// use flexi_logger::{
//   detailed_format, AdaptiveFormat, Duplicate, FileSpec, FlexiLoggerError, Logger, WriteMode,
// };

// use entity::sea_orm;
// use migration::MigrationTrait;
// use sea_orm::{entity::*, query::*};
// use sea_orm_rocket::{Connection, Database};

mod cors;
mod db;
mod helmet;
mod logger;
mod routes;

// TODO Restrict access to specific hosts, idea from https://www.npmjs.com/package/host-validation

#[launch]
fn rocket() -> _ {
  // pretty_env_logger::init();
  trace!("Initializing logger (file, stdout, stderr)..."); // Doesn't print
                                                           // init_logger().expect("Unable to initialize logger");
  info!("Loaded logger");
  trace!("Building rocket...");
  rocket::build()
    .mount("/", routes![index, options_cors])
    .mount("/devices/", crate::routes::devices::routes())
    .mount("/records/", crate::routes::records::routes())
    .attach(cors::CORS)
    .attach(helmet::Helmet)
    .attach(Shield::default())
  // .attach(logger::Log::new(logger::When::Both)) // Always put last, when running on_response at least, first if you want "pure" request
}

// fn init_logger() -> Result<(), FlexiLoggerError> {
//   // TODO Log at trace or info level in file?
//   Logger::try_with_env_or_str("info")?
//     .log_to_file(FileSpec::default().directory("logs"))
//     .duplicate_to_stderr(Duplicate::Error)
//     .duplicate_to_stdout(Duplicate::Trace)
//     .write_mode(WriteMode::Direct)
//     .format_for_files(detailed_format)
//     .format_for_stderr(detailed_format)
//     .adaptive_format_for_stdout(AdaptiveFormat::Default)
//     .print_message()
//     .create_symlink("current.log") //Do in term `tail -F current.log`
//     .start()?;
//   Ok(())
// }

// async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
//   let conn = &Db::fetch(&rocket).unwrap().conn;
//   let _ = migration::Migrator::up(conn, None).await;
//   Ok(rocket)
// }

#[get("/")]
fn index() -> Html<&'static str> {
  Html(include_str!("page/index.html"))
}

/// This route ensures that a response (not a 404) is sent to the client when making a pre-flight OPTIONs request
#[options("/<_..>")]
fn options_cors() {}
