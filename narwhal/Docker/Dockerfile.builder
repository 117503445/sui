FROM rust:1.75-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
COPY ./scripts/sources.list /etc/apt/sources.list
RUN apt-get update && apt-get install -y cmake clang

ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

COPY ./scripts/config.toml /root/.cargo/config

WORKDIR /workspace