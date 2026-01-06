from docker.io/clux/muslrust:stable AS build

COPY . /build
WORKDIR /build
RUN cargo build --release --no-default-features

FROM scratch

LABEL org.opencontainers.image.source="https://github.com/skutter-de/SaveSyncd"
LABEL org.opencontainers.image.description="A daemon for SaveSync, a tool to synchronize Nintendo 3DS savegames"
LABEL org.opencontainers.image.licenses="GPL-3.0-or-later"

ENV XDG_CONFIG_HOME=/config
ENV XDG_DATA_HOME=/data

COPY --from=build /build/target/x86_64-unknown-linux-musl/release/SaveSyncd /

CMD ["/SaveSyncd"]