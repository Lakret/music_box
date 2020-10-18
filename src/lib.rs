pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod spotify;
pub use spotify::authenticate as authenticate_spotify;
