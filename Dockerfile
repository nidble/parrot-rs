#############
## Builder ##
#############
FROM rust:1.49 as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev openssl libssl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN USER=app cargo new --bin parrot-rs

WORKDIR /parrot-rs
COPY ./Cargo.toml ./Cargo.toml
RUN touch ./src/lib.rs

# build the deps
RUN cargo build --target x86_64-unknown-linux-musl --release --bin parrot-rs
RUN rm src/*.rs
ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/parrot_rs*

# builds the app
RUN cargo build --target x86_64-unknown-linux-musl --release --bin parrot-rs

##############
## Runner ##
#############
FROM debian:buster-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y locales openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG APP=/app

EXPOSE 3030

ENV APP_USER=app:app \
    RUST_LOG=info

#RUN groupadd $APP_USER \
#    && useradd -g $APP_USER $APP_USER \
#    && mkdir -p ${APP}

COPY --from=builder /parrot-rs/target/x86_64-unknown-linux-musl/release/parrot-rs ${APP}/parrot-rs

RUN chown -R $APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./parrot-rs"]
