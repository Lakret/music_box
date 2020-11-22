# RUST_LOG=info cargo watch -s 'cargo run --bin webserver' & \
(cd webserver; RUST_LOG=info cargo run) & \
  npm --prefix webserver/assets install webserver/assets & \
  parcel watch --out-dir webserver/assets/dist  webserver/assets/index.html
# & cargo run --bin desktop
