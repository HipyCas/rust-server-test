use rocket::fairing::{Fairing, Info, Kind};
// use rocket::http::Header;
use rocket::data::ToByteUnit;
use rocket::tokio::io::AsyncReadExt;
use rocket::{Data, Request, Response};

#[allow(dead_code)]
pub enum When {
  Response,
  Request,
  Both,
}

pub struct Log {
  kind: Kind,
}

impl Log {
  pub fn new(when: When) -> Self {
    match when {
      When::Response => Log {
        kind: Kind::Response,
      },
      When::Request => Log {
        kind: Kind::Request,
      },
      When::Both => Log {
        kind: Kind::Response | Kind::Request,
      },
    }
  }
}

#[rocket::async_trait]
impl Fairing for Log {
  fn info(&self) -> Info {
    Info {
      name: "Implement some security",
      kind: self.kind,
    }
  }

  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    println!("=== RESPONSE ===");
    // response.remove_header("Server")
    println!("Request: {:#?}", request);
    println!("Response: {:#?}", response);
  }

  async fn on_request(&self, request: &mut Request<'_>, data: &mut Data<'_>) {
    // response.remove_header("Server")
    println!("=== REQUEST ===");
    println!("Request: {:#?}", request);
    // let mut red = Vec::<u8>::new();
    // let mut stream = data.peek(1.megabytes());
    // stream.read(&mut red);
    println!(
      "Data: {:?}",
      String::from_utf8(data.peek(1_000_000).await.to_vec())
    ); // TODO Figure out how to print
  }
}
