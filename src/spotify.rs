use std::collections::HashMap;
use std::env;
use std::fs;

use dotenv::dotenv;
use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};
use rspotify::model::artist::FullArtist;

use crate::Result;

/** Authenticates and returns Spotify client.

Users 2 files for authentication:

  - `.env` file (required), which should be an environment variables file
  corresponding to the following template:

    ```
    RSPOTIFY_CLIENT_ID=...
    RSPOTIFY_CLIENT_SECRET=...
    RSPOTIFY_REDIRECT_URI=...
    ```

    These should correspond to your Spotify application; you can create
    one at [Spotify developer dashboard](https://developer.spotify.com/dashboard/).

    You can set redirect URI to something like `http://localhost:8000`.

  - `.token` file (optional), containing Spotify `TokenInfo` serialized as bincode.
  This file will be created automatically if not found; the user will be prompted
  to follow the authentication link, and paste the code from the redirect link.
*/
pub fn authenticate() -> Result<Spotify> {
  let raw_client_credentials = read_env_variables()?;
  let token_info = match fs::read(".token") {
    Ok(serialized_token) => read_existing_token(&serialized_token[..]),
    Err(error) => match error.kind() {
      std::io::ErrorKind::NotFound => request_and_save_user_tokens(&raw_client_credentials),
      _ => Err(error.into()),
    },
  }?;

  let client_credentials = SpotifyClientCredentials::default()
    .client_id(&raw_client_credentials.client_id)
    .client_secret(&raw_client_credentials.client_secret)
    .token_info(token_info)
    .build();

  let spotify = Spotify::default()
    .client_credentials_manager(client_credentials)
    .build();

  Ok(spotify)
}

/// Returns all followed artists for the current user.
pub fn all_followed_artists(spotify: &Spotify) -> Result<Vec<FullArtist>> {
  let mut all_artists: Vec<FullArtist> = vec![];
  let mut next = None;

  loop {
    let mut artists = spotify.current_user_followed_artists(Some(50), next)?;

    next = artists.artists.items.last().map(|artist| artist.id.clone());
    all_artists.append(&mut artists.artists.items);

    if next.is_none() {
      break;
    };
  }

  Ok(all_artists)
}

// ------------------------
// Helpers
// ------------------------

#[derive(Clone, Debug)]
struct RawSpotifyClientCredentials {
  client_id: String,
  client_secret: String,
  redirect_uri: String,
}

/// Reads client id, secret and redirect URI from `.env` file.
fn read_env_variables() -> Result<RawSpotifyClientCredentials> {
  dotenv().ok();

  let env_vars = env::vars().collect::<HashMap<_, _>>();
  let client_id = env_vars
    .get("RSPOTIFY_CLIENT_ID")
    .ok_or("missing RSPOTIFY_CLIENT_ID in env")?
    .to_string();
  let client_secret = env_vars
    .get("RSPOTIFY_CLIENT_SECRET")
    .ok_or("missing RSPOTIFY_CLIENT_SECRET in env")?
    .to_string();
  let redirect_uri = env_vars
    .get("RSPOTIFY_REDIRECT_URI")
    .ok_or("missing RSPOTIFY_REDIRECT_URI in env")?
    .to_string();

  Ok(RawSpotifyClientCredentials {
    client_id,
    client_secret,
    redirect_uri,
  })
}

const SPOTIFY_SCOPES: &str = "user-read-email user-read-private user-top-read \
  user-read-recently-played user-follow-read user-library-read \
  user-read-currently-playing user-read-playback-state \
  user-read-playback-position playlist-read-collaborative \
  playlist-read-private user-follow-modify user-library-modify \
  user-modify-playback-state playlist-modify-public \
  playlist-modify-private ugc-image-upload";

/// Generates an OAuth2 challenge link, and asks the user to enter the received code.
/// If the token generation is successful, saves the token info in a `.token` file.
fn request_and_save_user_tokens(credentials: &RawSpotifyClientCredentials) -> Result<TokenInfo> {
  let oauth = SpotifyOAuth::default()
    .redirect_uri(&credentials.redirect_uri)
    .client_id(&credentials.client_id)
    .client_secret(&credentials.client_secret)
    .scope(SPOTIFY_SCOPES)
    .build();

  let authorize_url = oauth.get_authorize_url(None, None);
  println!("Please, authorize here:\n\t{}", authorize_url);

  println!("Paste your code here (you can ignore state):");
  let mut code = String::new();
  std::io::stdin().read_line(&mut code)?;
  let code = code.trim_end();

  let token_info = oauth
    .get_access_token(code)
    .ok_or("no access token info received")?;

  let serialized_token_info = bincode::serialize(&token_info)?;
  fs::write(".token", serialized_token_info)?;
  println!("Saved token info to .token file.");

  Ok(token_info)
}

/// Reads a serialized user token.
fn read_existing_token(serialized_token: &[u8]) -> Result<TokenInfo> {
  bincode::deserialize(serialized_token).map_err(|err| err.into())
}
