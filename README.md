# rust-hello-world
Rocket driven hello world web


### Docker setup:

#### Deps image: rust-hello-world/deps:*

Image where `dummy_app` is copyed to and dependencies are fetched. This way pre-cached dev/release images can just run build/check on the already retrieved dependencies.

 * Based on: nulldevelopment/rusty-nightly:2019-04-14
 * Folder: etc/docker/rust-deps/
 * Build: `make build-deps-image`

#### Development pre-cached deps image: rust-hello-world/dev-deps:*

By splitting dev/release dep images, hopefully development will be easier: dev one is used for handling local development while release one will be used to help with creating releases

 * Based on: rust-hello-world/deps:latest
 * Folder: etc/docker/rust-dev-deps/
 * Build: `make build-dev-deps-image`

#### Release pre-cached deps image: rust-hello-world/release-deps:*

 * Based on: rust-hello-world/deps:latest
 * Folder: etc/docker/rust-release-deps/
 * Build: `make build-release-deps-image`

