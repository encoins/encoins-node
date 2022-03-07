FROM rust:1.57 as builder

WORKDIR /usr/src/encoins
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/encoins /usr/local/bin/encoins
CMD ["encoins"]
