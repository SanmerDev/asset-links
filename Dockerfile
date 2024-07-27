FROM --platform=$BUILDPLATFORM rust:1.80-alpine AS builder
COPY . /rust/asset-links
WORKDIR /rust
RUN set -ex \
    && apk add libc-dev \
    && cargo install --path asset-links

FROM --platform=$TARGETPLATFORM alpine AS dist
COPY --from=builder /usr/local/cargo/bin/asset-links /usr/local/bin/asset-links
EXPOSE 8080
ENTRYPOINT ["asset-links"]