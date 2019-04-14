FROM rust-hello-world/deps:latest

ENV TARGET=x86_64-unknown-linux-musl
ENV BUILD_DIR=/target/x86_64-unknown-linux-musl/release/

WORKDIR /dummy_app
RUN ~/.cargo/bin/cargo build --release --target=${TARGET};


ONBUILD RUN mkdir /src /app
ONBUILD COPY . /src
ONBUILD WORKDIR /src

ONBUILD RUN ~/.cargo/bin/cargo test --release --target=${TARGET}; \
    ~/.cargo/bin/cargo build --release --target=${TARGET};

# Copy the "interesting" files into /app.
ONBUILD RUN find ${BUILD_DIR} \
                -regextype egrep \
                # The interesting binaries are all directly in ${BUILD_DIR}.
                -maxdepth 1 \
                # Well, binaries are executable.
                -executable \
                # Well, binaries are files.
                -type f \
                # Filter out tests.
                ! -regex ".*\-[a-fA-F0-9]{16,16}$" \
                # Copy the matching files into /app.
                -exec cp {} /app \;

ONBUILD RUN echo "The following files will be copied to the runtime image: $(ls /app)"