FROM rust:bookworm AS build
COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --release

FROM debian:bookworm-slim AS campus-api
COPY --from=build ./target/release/campus-api /app/campus-api
WORKDIR /app/data
EXPOSE 8080
ENTRYPOINT ["/app/campus-api"]