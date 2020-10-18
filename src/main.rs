extern crate rspotify;

use music_box::authenticate_spotify;
use music_box::Result;

fn main() -> Result<()> {
  let spotify = authenticate_spotify()?;

  spotify.start_playback(None, None, None, None, None)?;
  std::thread::sleep(std::time::Duration::from_secs(5));
  spotify.pause_playback(None)?;

  Ok(())
}
