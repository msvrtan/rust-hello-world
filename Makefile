SHELL := /bin/bash # Use bash syntax

.PHONY=*

CURRENT_DATE=`date +'%Y.%m.%d'`

build-deps-image:
	cd etc/docker/rust-deps/ && time -p docker build . -t rust-hello-world/deps:$(CURRENT_DATE)
	cd etc/docker/rust-deps/ && time -p docker build . -t rust-hello-world/deps:latest

build-dev-deps-image:
	cd etc/docker/rust-dev-deps/ && time -p docker build . -t rust-hello-world/dev-deps:$(CURRENT_DATE)
	cd etc/docker/rust-dev-deps/ && time -p docker build . -t rust-hello-world/dev-deps:latest

build-release-deps-image:
	cd etc/docker/rust-release-deps/ && time -p docker build . -t rust-hello-world/release-deps:$(CURRENT_DATE)
	cd etc/docker/rust-release-deps/ && time -p docker build . -t rust-hello-world/release-deps:latest
