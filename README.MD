## Coffee app
This is a small project, of testing out Rust based backend & front end tech.



## self hosting
The way i'd host this is with a web server connecting to dist, and a non-public web server.
Note: i use podman but you can use less secure options by replacing the word podman with whatever it is you use.
### Prerun
```bash
rustup add wasm32-unknown-unknown
cargo install trunk
```

### Deploying
```bash
git clone https://github.com/toastxc/coffee
cd coffee
# backend
podman-compose up -d
# frontend 
trunk build --release
```

### Coffee-shared
Coffee-shared is a shared dependancy between the backend and frontend, for the sake of easier compliation and docker issues [it has been moved to its own repository](https://github.com/toastxc/coffee-shared).