use std::path::PathBuf;

use crate::spotify;
use rspotify::blocking::client::Spotify;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct MusicLibrary {
  sources: Vec<MusicSource>,
}

impl MusicLibrary {
  pub fn new(sources: Vec<MusicSource>) -> MusicLibrary {
    MusicLibrary { sources }
  }
}

impl MusicLibrary {
  pub fn get_artists(&self) -> Result<Vec<Artist>> {
    let artists = self
      .sources
      .iter()
      .flat_map(|source| match source.get_artists() {
        Ok(artists) => artists,
        Err(_error) => vec![],
      })
      .collect::<Vec<_>>();

    Ok(artists)
  }
}

pub enum MusicSource {
  SpotifyClient(Spotify),
  LocalDirectory { name: String, path: PathBuf },
}

use MusicSource::*;

impl MusicSource {
  pub fn new_spotify_client() -> Result<MusicSource> {
    let client = spotify::authenticate()?;
    Ok(SpotifyClient(client))
  }

  pub fn new_local_dir_client(name: &str, path: &str) -> Result<MusicSource> {
    let name = name.to_string();
    let path = PathBuf::from(path);

    Ok(LocalDirectory { name, path })
  }

  pub fn get_artists(&self) -> Result<Vec<Artist>> {
    match self {
      SpotifyClient(client) => {
        let spotify_artists = spotify::all_followed_artists(&client)?;
        let artists = spotify_artists
          .into_iter()
          .map(|a| a.into())
          .collect::<Vec<Artist>>();

        Ok(artists)
      }
      LocalDirectory { .. } => {
        // TODO:
        Ok(vec![])
      }
    }
  }
}

#[derive(Debug)]
pub struct Artist {
  pub name: String,
  pub genres: Vec<String>,
  pub url: Option<String>,
  pub spotify_id: Option<String>,
  pub image_urls: Vec<String>,
}

impl From<rspotify::model::artist::FullArtist> for Artist {
  fn from(artist: rspotify::model::artist::FullArtist) -> Artist {
    Artist {
      name: artist.name,
      genres: artist.genres,
      url: artist
        .external_urls
        .values()
        .take(1)
        .map(|url| url.to_string())
        .collect::<Vec<_>>()
        .pop(),
      spotify_id: Some(artist.id),
      image_urls: artist.images.into_iter().map(|image| image.url).collect(),
    }
  }
}
