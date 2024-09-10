FROM rust:1.81-alpine@sha256:d6e876ca5fe200f4ac60312b95606f0b042699c4cf6a19493b7d2a2ebbfb337b AS build
RUN apk add build-base && rm -rf /var/cache/apk/*

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
RUN mv ./target/release/hyperbench /bin/hyperbench

FROM scratch AS final
ENTRYPOINT ["/bin/hyperbench"]
COPY --from=build /bin/hyperbench /bin/hyperbench

