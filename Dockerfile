FROM rust:1.83-alpine@sha256:838d384a1138fe1f2e448e3901bb3d23683570ba3dca581160880ffad760332b AS build
RUN apk add build-base && rm -rf /var/cache/apk/*

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
RUN mv ./target/release/hyperbench /bin/hyperbench

FROM scratch AS final
ENTRYPOINT ["/bin/hyperbench"]
COPY --from=build /bin/hyperbench /bin/hyperbench

