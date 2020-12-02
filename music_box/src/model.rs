use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::spotify;
use rspotify::client::Spotify;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct MusicLibrary {
  spotify: Option<Spotify>,
  dirs: Vec<LocalDir>,
}

#[derive(Debug, Clone)]
pub struct LocalDir {
  name: String,
  path: PathBuf,
}

impl MusicLibrary {
  pub fn new() -> MusicLibrary {
    MusicLibrary {
      spotify: None,
      dirs: vec![],
    }
  }

  pub fn set_spotify(&mut self, spotify: Spotify) -> &mut MusicLibrary {
    self.spotify = Some(spotify);
    self
  }

  pub fn add_dir(&mut self, name: &str, path: &str) -> &mut MusicLibrary {
    let local_dir = LocalDir {
      name: name.to_string(),
      path: PathBuf::from(path),
    };
    self.dirs.push(local_dir);
    self
  }

  pub fn sources(&self) -> Vec<Box<dyn MusicSource>> {
    let mut sources: Vec<Box<dyn MusicSource>> = vec![];

    for spotify in self.spotify.clone() {
      sources.push(Box::new(spotify.clone()));
    }

    for dir in self.dirs.clone() {
      sources.push(Box::new(dir.clone()));
    }

    sources
  }
}

impl MusicLibrary {
  pub fn get_artists(&self) -> Result<Vec<Artist>> {
    let artists = self
      .sources()
      .iter()
      .flat_map(|source| match source.get_artists() {
        Ok(artists) => artists,
        Err(_error) => vec![],
      })
      .collect::<Vec<_>>();

    Ok(artists)
  }
}

pub trait MusicSource {
  fn get_artists(&self) -> Result<Vec<Artist>>;
}

impl MusicSource for LocalDir {
  fn get_artists(&self) -> Result<Vec<Artist>> {
    // TODO:
    Ok(vec![])
  }
}

impl MusicSource for Spotify {
  fn get_artists(&self) -> Result<Vec<Artist>> {
    // TODO: limit and iterating
    let spotify_artists = self.current_user_followed_artists(1000, None)?;

    let artists = spotify_artists
      .artists
      .items
      .into_iter()
      .map(|a| a.into())
      .collect::<Vec<Artist>>();

    Ok(artists)
  }
}

#[derive(Debug, Serialize, Deserialize)]
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
