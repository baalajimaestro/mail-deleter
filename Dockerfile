FROM rust:bullseye as builder

RUN mkdir /app

COPY src /app/src
COPY Cargo.toml /app
WORKDIR /app
RUN cargo build --release

FROM gcr.io/distroless/cc:latest

COPY --from=builder /app/target/release/mail-deleter /
VOLUME /patterns.txt

CMD ["./mail-deleter"]