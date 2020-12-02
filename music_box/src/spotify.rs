use std::collections::HashMap;
use std::env;
use std::fs;

use dotenv::dotenv;
use rspotify::client::{Spotify, SpotifyBuilder};
use rspotify::model::artist::FullArtist;
use rspotify::oauth2::{Credentials, OAuth, Token};
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder};

use crate::Result;

pub fn new() -> Spotify {
  // TODO:
  // let oauth = OAuthBuilder::default()
  //   .redirect_uri(&raw_client_credentials.redirect_uri)
  //   .scope(SPOTIFY_SCOPES)
  //   .build()
  //   .expect("Cannot build OAuth");

  SpotifyBuilder::default()
    // TODO:
    // .credentials(client_credentials)
    // .oauth(oauth)
    .build()
    .expect("Cannot build Spotify")
}

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

  let client_credentials = CredentialsBuilder::default()
    .id(&raw_client_credentials.client_id)
    .secret(&raw_client_credentials.client_secret)
    // TODO:
    // .token(token_info)
    .build()
    .expect("Cannot build Client Credentials");

  let oauth = OAuthBuilder::default()
    .redirect_uri(&raw_client_credentials.redirect_uri)
    .scope(SPOTIFY_SCOPES)
    .build()
    .expect("Cannot build OAuth");

  let mut spotify = SpotifyBuilder::default()
    .credentials(client_credentials)
    .oauth(oauth)
    .build()
    .expect("Cannot build Spotify");

  // let token_info = match fs::read(".token") {
  //   Ok(serialized_token) => read_existing_token(&serialized_token[..]),
  //   Err(error) => match error.kind() {
  //     std::io::ErrorKind::NotFound => request_and_save_user_tokens(&raw_client_credentials),
  //     _ => Err(error.into()),
  //   },
  // }?;

  // TODO: restore token reuse
  request_and_save_user_tokens(&mut spotify)?;

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
fn request_and_save_user_tokens(spotify: &mut Spotify) -> Result<Token> {
  let authorize_url = spotify
    .get_authorize_url(false)
    .expect("Cannot get authorize URL");
  println!("Please, authorize here:\n\t{}", authorize_url);

  println!("Paste your code here (you can ignore state):");
  let mut code = String::new();
  std::io::stdin().read_line(&mut code)?;
  let code = code.trim_end();

  spotify.request_user_token(code)?;

  let token = spotify.token.clone().unwrap();
  let serialized_token = bincode::serialize(&token)?;
  fs::write(".token", serialized_token)?;
  println!("Saved token info to .token file.");

  Ok(token)
}

/// Reads a serialized user token.
fn read_existing_token(serialized_token: &[u8]) -> Result<Token> {
  bincode::deserialize(serialized_token).map_err(|err| err.into())
}
