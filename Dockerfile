FROM rust:1.78-buster as build
WORKDIR /queue_build
ADD Cargo.toml .
ADD src/ src
RUN cargo build --release

FROM debian:buster
COPY --from=build /queue_build/target/release/queue /bin/queue
EXPOSE 8080
CMD ["/bin/queue"]

