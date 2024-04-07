FROM rust:1.68-slim-buster AS build

WORKDIR /learn-actix

COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /learn-actix

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /learn-actix/target/release/learn-actix ./learn-actix

EXPOSE 80

CMD [ "./learn-actix" ]