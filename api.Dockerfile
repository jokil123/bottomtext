# FROM rust:1.65

# # WORKDIR /api
# COPY . .

# ENV API_PORT=3030
# # ENV PORT=8080

# RUN cargo install --path ./api --profile release

# # RUN cargo install trunk
# # RUN rustup target add wasm32-unknown-unknown


# EXPOSE 3030

# CMD api


FROM rust:1.65

# WORKDIR /api
COPY . .
# --profile release
# RUN cargo install --path ./api 

EXPOSE 3030

# CMD ["api"]
CMD ["cargo", "run", "--bin", "api"]