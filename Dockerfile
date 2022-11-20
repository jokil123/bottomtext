# compile tailwindcss
FROM node:14 as tailwind
WORKDIR /ui
COPY /ui .
RUN npm install -g tailwindcss
RUN NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./public/tailwind.css --minify


# Compile the Rust binary
FROM rust:1.65 as builder
# Copy cargo_home to avoid redownloading crates.io index
COPY cargo cargo
ENV CARGO_HOME=/cargo

# Install tooling
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-musl

# Copy the source code
COPY common common
COPY api api
COPY ui ui
COPY Cargo.lock Cargo.toml ./

# Build the server
RUN cargo build --release --package api --target x86_64-unknown-linux-musl

# Build the ui
COPY --from=0 ui/public/tailwind.css ui/public/tailwind.css
RUN cargo/bin/trunk build ui/index.html --release

# Run the server
FROM alpine:3.17 as runner
COPY --from=builder target/x86_64-unknown-linux-musl/release/api api/api
COPY --from=builder ui/dist ui/dist

# Expose the port
EXPOSE 3030

# Run the server
CMD ["api/api"]