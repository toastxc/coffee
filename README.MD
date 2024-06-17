## Coffee app
This is a small project, of testing out Rust based backend & front end tech. 



## self hosting
The way i'd host this is with a web server connecting to dist, and a non-public web server.
```bash
git clone https://github.com/toastxc/coffee
cd coffee
# backend
podman-compose up -d
# frontend 
trunk build --release