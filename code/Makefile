SHELL := /bin/bash # Use bash syntax

.PHONY=*


build:
	cd hello-rocket / && time -p cargo build

check:
	cd hello-rocket / && time -p cargo check

debug:
	cd hello-rocket / && RUST_BACKTRACE=1 time -p cargo run

format:
	cd hello-rocket / && time -p cargo fmt

release:
	cd hello-rocket / && time -p cargo build --release

run:
	cd hello-rocket / && time -p cargo run

test:
	cd hello-rocket / && time -p cargo test --tests

wip:
	git add . && git commit -am 'wip' && make build
