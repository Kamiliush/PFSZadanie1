# syntax=docker/dockerfile:1.2

FROM rust:1.66 AS build

COPY . /app
WORKDIR /app

RUN cargo build --release

COPY ./index.html ./target/release/

FROM gcr.io/distroless/cc-debian11

COPY --from=build /app/target/release/fibonacci_app /app/fibonacci_app
COPY --from=build /app/target/release/index.html /app

WORKDIR /app

CMD ["./fibonacci_app"]
