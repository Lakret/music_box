use music_box::*;

use rspotify::blocking::client::Spotify;
use rspotify::model::artist::FullArtist;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn play_local<P>(path: P) -> Result<()>
where
  P: AsRef<Path>,
{
  let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
  let file = File::open(path)?;
  let beep1 = stream_handle.play_once(BufReader::new(file)).unwrap();
  beep1.set_volume(0.5);

  thread::sleep(Duration::from_millis(2000));

  Ok(())
}

fn main() -> Result<()> {
  let spotify = MusicSource::new_spotify_client()?;
  let local_dir = MusicSource::new_local_dir_client("Test Dir", "music")?;

  let library = MusicLibrary::new(vec![spotify, local_dir]);
  // TODO: fetch local artists too
  println!("My artists: {:#?}", library.get_artists());

  // TODO: spotify playback
  // spotify.start_playback(None, None, None, None, None)?;
  // std::thread::sleep(std::time::Duration::from_secs(5));
  // spotify.pause_playback(None)?;

  // TODO: local playback
  // play_local("music/God Is Food/Τέχνη.mp3")?;
  // play_local("music/God Is Food/03 - Roads We Walk.flac")?;

  Ok(())
}
