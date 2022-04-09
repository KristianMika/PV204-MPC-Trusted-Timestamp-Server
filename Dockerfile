FROM scratch

ENV RUST_LOG = info
# TODO: release!!!
COPY ./target/x86_64-unknown-linux-musl/debug/timestamp_server /timestamp_server
EXPOSE 8080
VOLUME ["/config"]
CMD ["/timestamp_server"]
