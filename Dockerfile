FROM rust:1.82-alpine@sha256:466dc9924d265455aa73e72fd9cdac9db69ce6a988e6f0e6baf852db3485d97d AS build
RUN apk add build-base && rm -rf /var/cache/apk/*

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
RUN mv ./target/release/hyperbench /bin/hyperbench

FROM scratch AS final
ENTRYPOINT ["/bin/hyperbench"]
COPY --from=build /bin/hyperbench /bin/hyperbench

