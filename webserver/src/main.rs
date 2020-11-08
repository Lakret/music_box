use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer, Result};
use listenfd::ListenFd;
use std::path::PathBuf;

// #[get("/")]
// async fn hello() -> impl Responder {
//   HttpResponse::Ok().body("Hello world!")
// }

#[get("/")]
async fn index() -> Result<NamedFile> {
  let path: PathBuf = "./webserver/assets/dist/index.html".into();
  Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();
  let mut listenfd = ListenFd::from_env();

  let mut server = HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .service(index)
      .service(Files::new("/", "./webserver/assets/dist"))
  })
  .shutdown_timeout(5);

  server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(listener)?
  } else {
    server.bind("127.0.0.1:8000")?
  };

  server.run().await
}
