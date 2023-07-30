FROM rust:1.71-alpine as release

RUN apk add musl-dev

WORKDIR /usr/src/resource
COPY . .
RUN cargo install --locked --target-dir /target --path .

FROM alpine:3.18
COPY scripts /opt/resource
COPY --from=release /usr/local/cargo/bin/gitea-package /usr/local/bin/gitea-package
