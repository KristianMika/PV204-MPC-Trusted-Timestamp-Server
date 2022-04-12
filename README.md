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

*NOTE: In case the new build doesn't correctly remove the tag from the old image and you are getting the old code in the demo, remove all docker diks-tits images*
```bash
docker rmi --force diks-tits
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

The individual servers 1 - 3 are accessible on localhost:{8080-8082}.

### Usage

#### Postman
It is recommended to use [Postman](https://www.postman.com/) for request submission. A request collection that can be imported into Postman is located [here](Diks-tits-requests.postman_collection.json).

1. Trigger keygen by sending a POST request to localhost:8080/keygen
2. Use the localhost:8080/timestamp endpoint

#### wget
You can also use any client tool for request submission.

```bash
wget --method POST -O- 127.0.0.1:8080/keygen | /dev/null
```

## Encpoints

### KeyGen

- **POST /keygen** 
  - Triggers key generation
  - Submitted by admin, whose pubkey is certified, authority cert stored on every server
- **POST /init**
  - Servers receive the commitments and zero-knowledge proof here.
  - Used only by servers, request authentication using TLS and stored certificates.
- **POST /keygen_phase1**
  - Used for receptino of secret shares from group 1.
  - Used only by servers, request authentication using TLS and stored certificates.
- **POST /keygen_phase2**
  - Servers receive the groupkey, which is compared to the ones they computed.
  - Used only by servers, request authentication using TLS and stored certificates.

### Getters
- **GET /commitment**
  - Each server publishes its public commitments.
- **GET /pubkey**
  - Server's individual public key
- **GET /groupkey**
  - The computed groupkey

### Signing
- **POST /timestamp**
  - Publicly available endpoint, used by user for timestamp requests
- **POST /partial_signature**
  - Used by servers requesting a partial signature

## Current Implementation State
- For commitment index synchronization reasons, only one server can be used for requesting the /timestamp endpoint. (the index is not being synchronized right now)
- We are using a fixed subset of signers for signing
- The code quality is super bad. In fact, it could be used the next year for PA193 as a buggy code assignment (28 unwrap usages right now, 54 TODO occurences).
- Missing authentication - [TLS setup](https://actix.rs/docs/http2/).