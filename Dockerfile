from docker.io/clux/muslrust:stable AS build

COPY . /build
WORKDIR /build
RUN cargo build --release --no-default-features

FROM scratch

ENV XDG_CONFIG_HOME=/config
ENV XDG_DATA_HOME=/data

COPY --from=build /build/target/x86_64-unknown-linux-musl/release/SaveSyncd /

CMD ["/SaveSyncd"]
