FROM rust:1-slim-buster AS base

ENV USER=root

RUN apt-get update && apt-get install -y --no-install-recommends cmake apt-utils libssl-dev pkg-config build-essential libpq-dev

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY src /code/src

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --offline

FROM rust:1-slim-buster

COPY --from=builder /code/target/debug/speculare-server /usr/bin/speculare-server

RUN apt-get update && apt-get install -y libpq-dev

EXPOSE 8080

ENTRYPOINT [ "/usr/bin/speculare-server" ]
