FROM rust:1 AS build

RUN apt-get update && apt-get install libatk1.0-dev libgtk-3-dev libxdo-dev -y && apt-get clean

COPY . /build
WORKDIR /build
RUN cargo build --release

FROM debian:trixie

ENV XDG_CONFIG_HOME=/config
ENV XDG_DATA_HOME=/data

RUN apt-get update && apt-get install libatk1.0-dev libgtk-3-dev libxdo-dev -y && apt-get clean

COPY --from=build /build/target/release/SaveSyncd /

CMD ["/SaveSyncd"]