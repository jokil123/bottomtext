FROM rust:1.65

COPY . .

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Build the ui
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /ui

RUN trunk build --release

# Build the server
WORKDIR /api
RUN cargo install --path .

EXPOSE 3030

CMD ["api"]