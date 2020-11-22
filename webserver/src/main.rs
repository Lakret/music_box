use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Result};
use actix_web_static_files;
use listenfd::ListenFd;
use log::error;

use std::collections::HashMap;

use music_box::{MusicLibrary, MusicSource};

// for bundling assents inside the binary with `actix_web_static_files`.
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[get("/api/artists")]
async fn artists() -> HttpResponse {
  let data = r#"{"artists": ["Meshuggah"]}"#.to_string();

  HttpResponse::Ok()
    .content_type("application/json")
    .body(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();
  let mut listenfd = ListenFd::from_env();

  // TODO: need to use async HTTP client in music_box
  // let spotify = MusicSource::new_spotify_client().map_err(|err| {
  //   error!(
  //     "Got error while trying to initialize Spotify source: {}",
  //     err.to_string()
  //   );
  //   std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
  // })?;
  // let library = MusicLibrary::new(vec![spotify]);
  // println!("My artists: {:#?}", library.get_artists());

  let mut server = HttpServer::new(|| {
    let generated = generate();

    App::new()
      .wrap(Logger::default())
      // API endpoints
      .service(artists)
      // Auto-reloaded dev static assets & main page. You can access them
      // under `http://localhost:8000/dev`.
      // FIXME: The `index.html` file reloads properly,
      // but css and js changes are not yet auto-reloaded.
      .service(Files::new("/dev", "assets/dist/").index_file("index.html"))
      // Bundled static assets & main page (via actix_web_static_files)
      .service(actix_web_static_files::ResourceFiles::new("/", generated))
  })
  .shutdown_timeout(5);

  server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(listener)?
  } else {
    server.bind("127.0.0.1:8000")?
  };

  server.run().await
}
