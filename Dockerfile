FROM clux/muslrust:1.57.0 AS chef
RUN cargo install cargo-chef
WORKDIR /opt

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /opt/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM alpine:3.15 AS runtime
COPY --from=builder /opt/target/x86_64-unknown-linux-musl/release/supreme-funicular /usr/local/bin/supreme-funicular
ENTRYPOINT ["/usr/local/bin/supreme-funicular"]
