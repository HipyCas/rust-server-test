use rocket::fairing::{Fairing, Info, Kind};
// use rocket::http::Header;
use rocket::{Request, Response};

pub struct Helmet;

#[rocket::async_trait]
impl Fairing for Helmet {
  fn info(&self) -> Info {
    Info {
      name: "Implement some security",
      kind: Kind::Response,
    }
  }

  async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
    response.remove_header("Server")
  }
}
