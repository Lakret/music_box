use music_box::*;
use serde::{Deserialize, Serialize};
use web_view::*;

fn init_music_box() -> Result<MusicLibrary> {
  let spotify = MusicSource::new_spotify_client()?;
  let local_dir = MusicSource::new_local_dir_client("Test Dir", "../music")?;

  let library = MusicLibrary::new(vec![spotify, local_dir]);
  Ok(library)
}

#[derive(Serialize, Deserialize)]
struct State {
  artists: Vec<Artist>,
}

fn main() -> Result<()> {
  let library = init_music_box()?;
  let artists = library.get_artists()?;

  let state = State { artists };

  web_view::builder()
    .title("Music Box")
    // TODO: figure out how it should work without webpack dev server
    // .content(Content::Html(include_str!("../../ui/static/index.html")))
    .content(Content::Url("http://localhost:8000"))
    .size(800, 600)
    .resizable(true)
    .debug(true)
    .user_data(state)
    .invoke_handler(|webview, arg| {
      match arg {
        "init" => {
          println!("Called init");

          let state = webview.user_data();
          let state_json = serde_json::to_string(state);

          let js_command = format!("alert({:?})", &state_json);
          webview.eval(&js_command)?;
        }
        _ => {}
      }

      Ok(())
    })
    .run()
    .unwrap();

  Ok(())
}
