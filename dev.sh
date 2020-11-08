# RUST_LOG=info cargo watch -s 'cargo run --bin webserver' & \
RUST_LOG=info cargo run --bin webserver & \
  parcel watch --out-dir webserver/assets/dist  webserver/assets/index.html
# & cargo run --bin desktop
