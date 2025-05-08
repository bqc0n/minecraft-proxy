FROM rust:1.86.0-alpine3.21 as builder

RUN apk --no-cache add musl-dev

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ src/

RUN cargo build --release

RUN strip target/release/simple-minecraft-proxy -o main

FROM gcr.io/distroless/static-debian12:nonroot
USER nonroot

WORKDIR /app

COPY --from=builder /app/main /app/main


ENTRYPOINT [ "/app/main", "/app/config.yaml" ]