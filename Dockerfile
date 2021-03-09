FROM rustlang/rust:nightly

WORKDIR /usr/src/app

RUN cargo install cargo-watch

COPY . .
