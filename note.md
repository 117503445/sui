narwhal/Docker/README.md

docker compose -f docker-compose.yml up --build

pacman -Syu --noconfirm clang cmake

export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

cargo build --bin narwhal-node

cargo build --profile release --bin narwhal-node
cargo build --profile release --features=benchmark --bin narwhal-benchmark-client