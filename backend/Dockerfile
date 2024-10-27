# Web APIサーバのビルド
FROM rust:1.70-slim-bullseye as server

WORKDIR /usr/src/myapp

RUN apt-get update && \
    apt-get install -y libpq-dev &&\
    apt-get install -y wait-for-it

COPY . .

# E2Eテストを実行するクライアントのビルド
FROM rust:1.70-slim-bullseye as client

WORKDIR /usr/src/myapp

RUN apt-get update && \
    apt-get install -y wait-for-it

COPY e2e/ .
