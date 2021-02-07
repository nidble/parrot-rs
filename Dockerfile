FROM rust:1.49 as builder

RUN USER=root cargo new --bin parrot-rs

WORKDIR /parrot-rs
COPY ./Cargo.toml ./Cargo.toml
RUN touch ./src/lib.rs
RUN cargo build --release --bin parrot-rs
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/parrot_rs*
RUN cargo build --release --bin parrot-rs


FROM debian:buster-slim

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y locales openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG APP=/usr/src/app

EXPOSE 3030

ENV APP_USER=appuser \
    RUST_LOG=info

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /parrot-rs/target/release/parrot-rs ${APP}/parrot-rs

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./parrot-rs"]