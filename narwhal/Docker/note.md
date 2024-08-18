cd narwhal/Docker
docker compose -f docker-compose.yml up --build

docker compose -f docker-compose.yml up

docker run -it --rm rust:1.75-bullseye bash



docker build -t sui-builder -f narwhal/Docker/Dockerfile.builder .



docker run -it -d --rm -v $PWD/Cargo.toml:/workspace/Cargo.toml -v $PWD/Cargo.lock:/workspace/Cargo.lock -v $PWD/consensus:/workspace/consensus -v $PWD/crates:/workspace/crates -v $PWD/sui-execution:/workspace/sui-execution -v $PWD/narwhal:/workspace/narwhal -v $PWD/external-crates:/workspace/external-crates -v $PWD/.target-docker:/workspace/target -v $PWD/.cargo-docker:/usr/local/cargo/registry sui-builder bash

docker run -it --rm -v $PWD/Cargo.toml:/workspace/Cargo.toml -v $PWD/Cargo.lock:/workspace/Cargo.lock -v $PWD/consensus:/workspace/consensus -v $PWD/crates:/workspace/crates -v $PWD/sui-execution:/workspace/sui-execution -v $PWD/narwhal:/workspace/narwhal -v $PWD/external-crates:/workspace/external-crates -v $PWD/.target-docker:/workspace/target sui-builder cargo build --profile release --bin narwhal-node

cargo build --profile release --bin narwhal-node && cargo build --profile release --features=benchmark --bin narwhal-benchmark-client