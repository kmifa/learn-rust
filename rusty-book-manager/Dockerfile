# マルチステージビルドを使用し、Rustのプログラムをビルドする
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app

ARG DATABESE_URL
ENV DATABASE_URL=${DATABESE_URL}

COPY . .
RUN cargo build --release

# 不要なソフトウェアを同梱する必要はないので、軽量なbookworm-slimを使用
FROM debian:bookworm-slim
WORKDIR /app
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

ENV PORT=8080
EXPOSE $PORT
ENTRYPOINT [ "./target/release/app" ]
