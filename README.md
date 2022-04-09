# DIKS TITS 

**Di**stributed **K**ey **S**igning **Ti**mes**t**amp **S**erver

## Build

### Static Linking (For the Docker demo)

It is necessary to link the C libraries statically to run the final binary in a Docker container.
The build uses [rust-musl-builder](https://github.com/emk/rust-musl-builder).

```bash
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
```
### Regular build
```bash
cargo build --release
```

## Demo

*Prerequisites: Build*

1. Build the Docker image
```bash
docker build . --tag diks-tits:latest
```
2. Run Docker compose
```bash
docker-compose up
```