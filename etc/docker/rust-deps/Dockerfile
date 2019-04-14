FROM nulldevelopment/rusty-nightly:2019-04-14

ENV TARGET=x86_64-unknown-linux-musl

RUN mkdir /target /dummy_app
COPY . /dummy_app

WORKDIR /dummy_app
RUN ~/.cargo/bin/cargo fetch --target=${TARGET};

CMD ["sleep", "1d"]