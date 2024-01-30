VERSION --try --global-cache 0.7
PROJECT applied-knowledge-systems/terraphim-project
IMPORT ./desktop AS desktop
IMPORT github.com/earthly/lib/rust AS rust
FROM ubuntu:20.04

ARG TARGETARCH
ARG TARGETOS
ARG TARGETPLATFORM
ARG --global tag=$TARGETOS-$TARGETARCH
ARG --global TARGETARCH
IF [ "$TARGETARCH" = amd64 ]
    ARG --global ARCH=x86_64
ELSE
    ARG --global ARCH=$TARGETARCH
END

WORKDIR /code

pipeline:
  BUILD desktop+build
  BUILD +fmt
  BUILD +lint
  BUILD +test
  BUILD +build


# Creates a `./artifact/bin` folder with all binaries
build-all:
  BUILD +build # x86_64-unknown-linux-gnu
  BUILD +cross-build --TARGET=x86_64-unknown-linux-musl
  BUILD +cross-build --TARGET=armv7-unknown-linux-musleabihf
  BUILD +cross-build --TARGET=aarch64-unknown-linux-musl
  # Errors
  # BUILD +cross-build --TARGET=aarch64-apple-darwin

docker-all:
  BUILD --platform=linux/amd64 +docker-musl --TARGET=x86_64-unknown-linux-musl
  BUILD --platform=linux/arm/v7 +docker-musl --TARGET=armv7-unknown-linux-musleabihf
  BUILD --platform=linux/arm64/v8 +docker-musl --TARGET=aarch64-unknown-linux-musl

install:
  FROM rust:1.75.0-buster
  RUN apt-get update -qq
  RUN apt install -y musl-tools musl-dev
  RUN update-ca-certificates
  RUN rustup component add clippy
  RUN rustup component add rustfmt
  RUN cargo install cross
  DO rust+INIT --keep_fingerprints=true

source:
  FROM +install
  COPY --keep-ts Cargo.toml Cargo.lock ./
  COPY --keep-ts --dir terraphim-server desktop crates terraphim_types  ./
  COPY desktop+build/dist /code/terraphim-server/dist
  DO rust+CARGO --args=fetch

cross-build:
  FROM +source
  ARG --required TARGET
  DO rust+SET_CACHE_MOUNTS_ENV
  WITH DOCKER
    RUN --mount=$EARTHLY_RUST_CARGO_HOME_CACHE --mount=$EARTHLY_RUST_TARGET_CACHE  cross build --target $TARGET --release
  END
  DO rust+COPY_OUTPUT --output=".*" # Copies all files to ./target
   RUN ./target/$TARGET/release/terraphim-server --version
  SAVE ARTIFACT ./target/$TARGET/release/terraphim-server AS LOCAL artifact/bin/terraphim_server-$TARGET

build:
  FROM +source
  DO rust+CARGO --args="build --offline --release" --output="release/[^/\.]+"
  RUN ./target/release/terraphim_server --version
  SAVE ARTIFACT ./target/release/terraphim_server AS LOCAL artifact/bin/terraphim_server-$TARGET

test:
  FROM +source
  DO rust+CARGO --args="test"

fmt:
  FROM +source
  DO rust+CARGO --args="fmt --check"

lint:
  FROM +source
  DO rust+CARGO --args="clippy --no-deps --all-features --all-targets"

docker-musl:
  FROM alpine:3.18
  # You can pass multiple tags, space separated
  # SAVE IMAGE --push ghcr.io/applied-knowledge-systems/terraphim-fastapiapp:bionic
  ARG tags="ghcr.io/applied-knowledge-systems/terraphim-server:latest"
  ARG --required TARGET
  COPY --chmod=0755 --platform=linux/amd64 (+cross-build/terraphim_server --TARGET=${TARGET}) /terraphim_server
  RUN /terraphim_server --version
  ENV TERRAPHIM_SERVER_HOSTNAME="127.0.0.1:8000"
  ENV TERRAPHIM_SERVER_API_ENDPOINT="http://localhost:8000/api"
  EXPOSE 8000
  ENTRYPOINT ["/terraphim_server"]
  SAVE IMAGE --push ${tags}

docker-aarch64:
  FROM rust:latest

  RUN apt update && apt upgrade -y
  RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross

  RUN rustup target add aarch64-unknown-linux-gnu
  RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

  WORKDIR /app

  ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
      CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
      CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

  CMD ["cargo", "build","--release","--target", "aarch64-unknown-linux-gnu"]

docker-slim:
    FROM debian:buster-slim
    COPY +build/terraphim_server terraphim_server
    EXPOSE 8000
    ENTRYPOINT ["./terraphim_server"]
    SAVE IMAGE aks/terraphim_server:buster