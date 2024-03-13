# APIRUST 
[API based on the ga database model]

# endpoints
- GET http://localhost:8090/api/post
- - GET http://localhost:8090/api/user

# installation 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo run
cargo build --release
