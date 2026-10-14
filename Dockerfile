FROM rust:1.99 AS backend

WORKDIR /app

RUN rustup target add "$(uname -m)-unknown-linux-musl" && \
    apt-get update && \
    apt-get install -y --no-install-recommends musl-tools musl-dev && \
    rm -rf /var/lib/apt/lists/*

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc
ENV CC_x86_64_unknown_linux_musl=musl-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc
ENV CC_aarch64_unknown_linux_musl=musl-gcc

COPY . .

RUN cargo fetch --locked && \
    cargo build --locked --release --bin dsa-server --target "$(uname -m)-unknown-linux-musl" && \
    cp "/app/target/$(uname -m)-unknown-linux-musl/release/dsa-server" /usr/local/bin/dsa-server

FROM node:26 AS frontend

WORKDIR /app

COPY ./web/package.json ./web/package-lock.json ./
RUN npm ci --include=dev
COPY ./web .

RUN npm run build && \
    mkdir -p /var/www/html && \
    cp /app/dist/. /var/www/html -r

FROM alpine:latest

WORKDIR /app

COPY --from=backend /usr/local/bin/dsa-server ./dsa-server
COPY --from=frontend /var/www/html ./dist
COPY ./resources ./resources

EXPOSE 8889

CMD ["./dsa-server"]
