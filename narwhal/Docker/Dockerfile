# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
# FROM rust:1.75-bullseye AS builder
# ARG PROFILE=release
# ARG GIT_REVISION
# ENV GIT_REVISION=$GIT_REVISION
# WORKDIR "$WORKDIR/sui"
# COPY ./scripts/sources.list /etc/apt/sources.list
# RUN apt-get update && apt-get install -y cmake clang

# ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
# ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

# COPY Cargo.toml Cargo.lock ./
# COPY consensus consensus
# COPY crates crates
# COPY sui-execution sui-execution
# COPY narwhal narwhal
# COPY external-crates external-crates
# COPY ./scripts/config.toml /root/.cargo/config

# RUN cargo build --profile ${PROFILE} --bin narwhal-node
# RUN cargo build --profile ${PROFILE} --features=benchmark --bin narwhal-benchmark-client

# Production Image
# FROM debian:bookworm-slim AS runtime
FROM debian:bullseye-slim AS runtime
COPY ./scripts/sources.list /etc/apt/sources.list
# COPY ./scripts/debian.sources /etc/apt/sources.list.d/debian.sources
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev
ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so
ARG PROFILE=release
WORKDIR "$WORKDIR/sui"

# Copy the Narwhal node binary to bin folder
COPY ./.target-docker/${PROFILE}/narwhal-node bin/

# This is used for testing a cluster by generating load.
COPY ./.target-docker/${PROFILE}/narwhal-benchmark-client bin/

# Copy the entry point file
COPY narwhal/Docker/entry.sh ./

# Now add the entry point
CMD ./entry.sh

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# CMD bash