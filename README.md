# DiKS TiTS 

**Di**stributed **K**ey **S**igning **Ti**mes**t**amp **S**erver

## Build

### Static Linking (For the Docker demo)

It is necessary to link the C libraries statically to run the final binary in a Docker container.
The build uses [rust-musl-builder](https://github.com/emk/rust-musl-builder).

```bash
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build # TODO: add --release before the assignment submission
```
*NOTE: In case the new build doesn't correctly remove the tag from the old image, remove all docker big-tits images*
```bash
docker rmi --force diks-tits
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
docker-compose down
docker-compose up
```
3. (optionally) remove previous containers
- used for development
```bash
docker-compose rm --stop -v
```

The individual servers 1 - 3 are accessible on localhost:{8081-8083}.

### Usage

#### Trigger Keygeneration
```bash
wget --method POST -O- 127.0.0.1:8081/keygen | /dev/null
```