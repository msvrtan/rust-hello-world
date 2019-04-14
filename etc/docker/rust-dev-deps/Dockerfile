FROM rust-hello-world/deps:latest

ENV TARGET=x86_64-unknown-linux-musl

WORKDIR /dummy_app
RUN ~/.cargo/bin/cargo build --target=${TARGET}; \
    ~/.cargo/bin/cargo check --target=${TARGET};

CMD ["sleep", "1d"]