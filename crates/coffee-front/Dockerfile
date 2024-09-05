FROM rust:latest
USER 0:0
WORKDIR /home/rust/src

RUN apt-get update && apt-get install -y make pkg-config libssl-dev

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
#COPY ./src ./src 
#COPY Cargo.* ./ 
COPY ./ ./ 
#RUN trunk build --release ./index.html

CMD ["trunk", "serve", "--address", "0.0.0.0","--release", "./index.html" ]
