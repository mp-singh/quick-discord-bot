FROM rust as build

WORKDIR /usr/src/
COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /usr/src/target/release/quick-discord-bot /bot/quick-discord-bot
WORKDIR /bot

RUN apt-get update && apt-get install libssl-dev ca-certificates -y && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/lists /bot/lists 

ENTRYPOINT ["/bot/quick-discord-bot"]
