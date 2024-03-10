# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye-slim as builder

ADD http://ftp.cz.debian.org/debian/pool/main/u/unzip/unzip_6.0-26+deb11u1_amd64.deb unzip.deb
RUN dpkg -i unzip.deb
# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos

ADD https://github.com/protocolbuffers/protobuf/releases/download/v25.1/protoc-25.1-linux-x86_64.zip protoc.zip
RUN unzip protoc.zip
RUN cp bin/protoc /usr/local/cargo/bin

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo build --release

FROM rustlang/rust:nightly-bullseye-slim as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/udp-server /app/

COPY --from=builder /app/data/environment.gltf /app/data/environment.gltf
COPY --from=builder /app/Cargo.toml /app/

WORKDIR /app

# Set any required env variables and
EXPOSE 8080/udp
# Run the server
CMD ["/app/udp-server"]
