FROM rust:1.57

WORKDIR /usr/src/encoins
COPY . .

RUN cargo install --path .

CMD ["encoins", "10", "0"]
