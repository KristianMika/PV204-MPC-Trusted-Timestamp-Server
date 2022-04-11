#!/bin/bash

set -e

cargo fmt
docker rmi --force diks-tits || 1
docker-compose rm --stop -v -f || 1
docker run --rm -it -v $(pwd):/home/rust/src ekidd/rust-musl-builder cargo build

docker build . --tag diks-tits:latest
