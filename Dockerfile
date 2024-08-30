FROM rust:1.80-alpine@sha256:1f5aff501e02c1384ec61bb47f89e3eebf60e287e6ed5d1c598077afc82e83d5 AS build
RUN apk add build-base && rm -rf /var/cache/apk/*

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
RUN mv ./target/release/hyperbench /bin/hyperbench

FROM scratch AS final
ENTRYPOINT ["/bin/hyperbench"]
COPY --from=build /bin/hyperbench /bin/hyperbench

