FROM rust:latest as builder

#ARG config=./Config.toml

RUN USER=root cargo new --bin holomon-server
WORKDIR ./holomon-server
COPY ./Cargo.toml ./Cargo.toml
COPY ./Config.toml ./Config.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/*
RUN cargo build --release

FROM ubuntu
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3030

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /holomon-server/target/release/holomon-server ${APP}/holomon-server
COPY --from=builder /holomon-server/Config.toml ${APP}/Config.toml
#COPY --from=builder /holomon-server/target/release/Config.toml ${APP}/Config.toml
#COPY ./Config.toml ${APP}/Config.toml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ./holomon-server -c ./Config.toml
