FROM rust:1-bookworm

WORKDIR /app

COPY backend/ .

RUN cargo build --release && mkdir -p .local

EXPOSE 8989

CMD ["./target/release/bookmark-backend"]
