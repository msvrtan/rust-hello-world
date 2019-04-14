FROM rust-hello-world/release-deps:latest as builder

FROM alpine:latest
COPY --from=builder /app/ /app/

CMD ["/app/hello-rocket"]
