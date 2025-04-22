FROM rust:latest AS backend

WORKDIR /app

RUN rustup install nightly && \
    rustup default nightly && \
    rustup target add x86_64-unknown-linux-musl

RUN apt update && \
    apt install -y musl-tools musl-dev clang pkg-config lld

COPY . .

RUN cargo fetch && \
    cargo build --release --bin dsa-server --target x86_64-unknown-linux-musl && \
    cp /app/target/x86_64-unknown-linux-musl/release/dsa-server /usr/local/bin/dsa-server

FROM node:latest AS frontend

WORKDIR /app

COPY ./web .

RUN npm install && \
    npm run build && \
    mkdir -p /var/www/html && \
    cp /app/dist/. /var/www/html -r

FROM alpine:latest

WORKDIR /app

COPY --from=backend /usr/local/bin/dsa-server ./dsa-server
COPY --from=frontend /var/www/html ./dist
COPY ./resources ./resources

EXPOSE 8889

CMD ["./dsa-server"]
