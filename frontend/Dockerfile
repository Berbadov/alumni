# syntax=docker/dockerfile:1.7
FROM rust:1.98 AS build
RUN rustup target add wasm32-unknown-unknown
RUN curl -fsSL \
    https://github.com/trunk-rs/trunk/releases/download/v0.22.0/trunk-x86_64-unknown-linux-gnu.tar.gz \
    | tar -xzf - -C /usr/local/bin
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch && rm -rf src
COPY index.html style.css Trunk.toml ./
COPY src ./src
RUN trunk build --release

FROM nginx:alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
