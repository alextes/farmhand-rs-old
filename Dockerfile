FROM rust as builder
WORKDIR app
COPY . .
RUN cargo build --release --bin farmhand

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/farmhand /usr/local/bin
ENTRYPOINT ["/usr/local/bin/farmhand"]
