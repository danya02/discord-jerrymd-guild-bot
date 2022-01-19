FROM rust:1.58.0

WORKDIR /app

COPY Cargo.toml .
COPY src src


RUN cargo install --path . --features=production
CMD ["discord-jerrymd-guild-bot"]
