FROM rust:1.70

RUN mkdir -p /app
WORKDIR /app

RUN apt-get update
RUN apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev
RUN apt-get install -y libasound2-dev
RUN apt-get install -y libudev-dev
RUN cargo install cargo-tarpaulin

COPY . /app

CMD ["bash", "-c", "cargo tarpaulin --output-dir ./target/debug/tarpaulin/ --out html && sleep 600"]
