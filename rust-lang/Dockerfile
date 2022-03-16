FROM rust:1.57

WORKDIR /rust-app

COPY . .

RUN cargo install cargo-criterion

CMD ["cargo", "criterion", "--output-format", "verbose"]