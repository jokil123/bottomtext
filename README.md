# Bottomtext

WHAT HOW?

## Setup

### Api

1. run app

   `cargo run`

### Ui

1. install trunk

   `cargo install trunk`

1. add wasm build target

   `rustup target add wasm32-unknown-unknown`

1. serve ui

   `trunk serve ui`

1. start tailwind

`NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./public/tailwind.css --minify --watch`

### run dockercontainer remotely

1. `docker-compose -H "ssh://<user>@<ip>" -f docker-compose.prod.yml up`

## Configuration

configuration for the ui is located in `ui/Trunk.toml`. The default ip for the ui is `0.0.0.0:8080`.

The api currently has no config and runs on `localhost:3030`, however it's requests are proxied by the ui and therefore does not need to be exposed tho the internet.
