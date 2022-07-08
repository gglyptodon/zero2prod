FROM rust:1.62.0
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV RUN_MODE production
ENTRYPOINT ["./target/release/zero2prod"]
